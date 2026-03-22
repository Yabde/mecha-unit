use bevy::prelude::*;
use crate::economy::components::*;
use crate::economy::resources::*;
use crate::core::components::Speed;

// 1. Spawner la base et les filons
pub fn spawn_economy_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // ---- BASE (Team 1) ---- Bas-gauche de la carte
    crate::factory::buildings::spawn_base(&mut commands, &mut meshes, &mut materials, Vec2::new(-1600.0, -1100.0), 1);

    // ---- CRISTAUX proches de la base joueur ----
    for i in 0..5 {
        let pos = Vec2::new(-1200.0 + (i as f32 * 60.0), -900.0 - (i as f32 * 40.0));
        crate::factory::resources::spawn_crystal(&mut commands, &mut meshes, &mut materials, pos, 300.0);
    }

    // ---- CRISTAUX zone neutre (centre) ----
    for i in 0..3 {
        let pos = Vec2::new(-100.0 + (i as f32 * 100.0), 50.0 - (i as f32 * 60.0));
        crate::factory::resources::spawn_crystal(&mut commands, &mut meshes, &mut materials, pos, 500.0);
    }

    // ---- BASE (Team 2 - IA) ---- Haut-droite de la carte
    crate::factory::buildings::spawn_base(&mut commands, &mut meshes, &mut materials, Vec2::new(1600.0, 1100.0), 2);

    // ---- CRISTAUX proches de la base IA ----
    for i in 0..5 {
        let pos = Vec2::new(1200.0 - (i as f32 * 60.0), 900.0 + (i as f32 * 40.0));
        crate::factory::resources::spawn_crystal(&mut commands, &mut meshes, &mut materials, pos, 300.0);
    }
}

// 2. Intelligence/Machine à état de l'Ouvrier
pub fn worker_ai(
    time: Res<Time>,
    mut commands: Commands,
    mut economy: ResMut<PlayerEconomy>,
    mut q_workers: Query<(Entity, &mut Transform, &mut Worker, &mut WorkerState, &Speed, &mut MineTimer), Without<Base>>,
    mut q_resources: Query<(Entity, &mut ResourceNode, &Transform), Without<Worker>>,
    q_bases: Query<(Entity, &Transform), With<Base>>,
) {
    for (worker_entity, mut transform, mut worker, mut state, speed, mut mine_timer) in q_workers.iter_mut() {
        match *state {
            WorkerState::Idle => {
                // S'il ne fait rien, on check s'il est plein. Si oui, il devrait sûrement rentrer à la base
                if worker.current_load >= worker.capacity {
                    if let Some((base_entity, _)) = q_bases.iter().next() {
                        *state = WorkerState::ReturningToBase(base_entity);
                    }
                }
            },
            
            WorkerState::MovingToResource(target_node) => {
                let Ok((_, _, node_transform)) = q_resources.get(target_node) else {
                    *state = WorkerState::Idle; // Le cristal a disparu !
                    continue;
                };
                
                let worker_pos = transform.translation.truncate();
                let target_pos = node_transform.translation.truncate();
                let direction = target_pos - worker_pos;
                let distance = direction.length();
                
                if distance < 30.0 {
                    // Arrivé, on commence à piocher !
                    *state = WorkerState::Mining(target_node);
                    mine_timer.0.reset();
                } else {
                    // On continue de marcher vers le cristal
                    let movement = direction.normalize() * speed.0 * time.delta_secs();
                    transform.translation += movement.extend(0.0);
                }
            },
            
            WorkerState::Mining(target_node) => {
                let Ok((_, mut node, _)) = q_resources.get_mut(target_node) else {
                    *state = WorkerState::Idle; // Cristal détruit par un copain ouvrier par exemple
                    continue;
                };
                
                mine_timer.0.tick(time.delta());
                
                if mine_timer.0.just_finished() {
                    // Collecte par "coup de pioche"
                    let gather_amount = 5.0_f32.min(worker.capacity - worker.current_load).min(node.amount);
                    worker.current_load += gather_amount;
                    node.amount -= gather_amount;
                    
                    if node.amount <= 0.0 {
                        commands.entity(target_node).despawn();
                    }
                    
                    if worker.current_load >= worker.capacity || node.amount <= 0.0 {
                        // Inventaire plein ou filon épuisé => Retour base
                        if let Some((base_entity, _)) = q_bases.iter().next() {
                            *state = WorkerState::ReturningToBase(base_entity);
                        } else {
                            *state = WorkerState::Idle;
                        }
                    }
                }
            },
            
            WorkerState::ReturningToBase(target_base) => {
                let Ok((_, base_transform)) = q_bases.get(target_base) else {
                    *state = WorkerState::Idle; // Base détruite (!?)
                    continue;
                };
                
                let worker_pos = transform.translation.truncate();
                let target_pos = base_transform.translation.truncate();
                let direction = target_pos - worker_pos;
                let distance = direction.length();
                
                if distance < 65.0 { 
                    // Dépose le chargement !
                    economy.crystals += worker.current_load as u32;
                    worker.current_load = 0.0;
                    
                    // L'ouvrier repart chercher le cristal le plus proche
                    if let Some((best_node, _, _)) = q_resources.iter().min_by(|(_, _, t_a), (_, _, t_b)| {
                        let pos_a = t_a.translation.truncate();
                        let pos_b = t_b.translation.truncate();
                        let da = worker_pos.distance_squared(pos_a);
                        let db = worker_pos.distance_squared(pos_b);
                        da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                    }) {
                        *state = WorkerState::MovingToResource(best_node);
                    } else {
                        *state = WorkerState::Idle; // Plus de cristaux sur la carte
                    }
                } else {
                    let movement = direction.normalize() * speed.0 * time.delta_secs();
                    transform.translation += movement.extend(0.0);
                }
            }
        }
    }
}
