use crate::layout::Layout;

use itertools::izip;

pub fn apply_attraction_force2_2d(layout: &mut Layout) {
    for (_edge, (n1, n2)) in layout.edges.iter().enumerate() {
        let n1_pos = layout.points.get(*n1);
        let n2_pos = layout.points.get(*n2);

        let n1_mass = { *layout.masses.get(*n1).unwrap() };
        let n2_mass = { *layout.masses.get(*n2).unwrap() };

        if let Some((n1_speed, n2_speed)) = layout.speeds.get_2_mut(*n1, *n2) {
            let dx = { *n1_pos.get(0).unwrap() - *n2_pos.get(0).unwrap() };
            let dy = { *n1_pos.get(1).unwrap() - *n2_pos.get(1).unwrap() };

            let dist = (dx * dx + dy * dy).sqrt();
            let dire_x = dx / dist;
            let dire_y = dy / dist;

            let diff = layout.settings.link_distance - dist;
            let param = diff * layout.settings.edge_strength;

            let target_mass_ratio = 1.0 / n1_mass;
            let source_mass_ratio = 1.0 / n2_mass;

            let dis_x = dire_x * param;
            let dis_y = dire_y * param;
            *n1_speed.get_mut(0).unwrap() += dis_x * target_mass_ratio;
            *n1_speed.get_mut(1).unwrap() += dis_y * target_mass_ratio;
            *n2_speed.get_mut(0).unwrap() -= dis_x * source_mass_ratio;
            *n2_speed.get_mut(1).unwrap() -= dis_y * source_mass_ratio;
        }
    }
}

pub fn apply_attraction_force2_3d(layout: &mut Layout) {
    for (_edge, (n1, n2)) in layout.edges.iter().enumerate() {
        let n1_pos = layout.points.get(*n1);
        let n2_pos = layout.points.get(*n2);

        let n1_mass = { *layout.masses.get(*n1).unwrap() };
        let n2_mass = { *layout.masses.get(*n2).unwrap() };

        if let Some((n1_speed, n2_speed)) = layout.speeds.get_2_mut(*n1, *n2) {
            let dx = { *n1_pos.get(0).unwrap() - *n2_pos.get(0).unwrap() };
            let dy = { *n1_pos.get(1).unwrap() - *n2_pos.get(1).unwrap() };
            let dz = { *n1_pos.get(2).unwrap() - *n2_pos.get(2).unwrap() };

            let dist = (dx * dx + dy * dy + dz * dz).sqrt();
            let dire_x = dx / dist;
            let dire_y = dy / dist;
            let dire_z = dz / dist;

            let diff = layout.settings.link_distance - dist;
            let param = diff * layout.settings.edge_strength;

            let target_mass_ratio = 1.0 / n1_mass;
            let source_mass_ratio = 1.0 / n2_mass;

            let dis_x = dire_x * param;
            let dis_y = dire_y * param;
            let dis_z = dire_z * param;
            *n1_speed.get_mut(0).unwrap() += dis_x * target_mass_ratio;
            *n1_speed.get_mut(1).unwrap() += dis_y * target_mass_ratio;
            *n1_speed.get_mut(2).unwrap() += dis_z * target_mass_ratio;
            *n2_speed.get_mut(0).unwrap() -= dis_x * source_mass_ratio;
            *n2_speed.get_mut(1).unwrap() -= dis_y * source_mass_ratio;
            *n2_speed.get_mut(2).unwrap() -= dis_z * source_mass_ratio;
        }
    }
}

pub fn apply_attraction_fruchterman_2d(layout: &mut Layout) {
    let k = &layout.settings.ka;
    let kr = &layout.settings.kr;
    for (_edge, (n1, n2)) in layout.edges.iter().enumerate() {
        let n1_pos = layout.points.get(*n1);
        let n2_pos = layout.points.get(*n2);

        if let Some((n1_speed, n2_speed)) = layout.speeds.get_2_mut(*n1, *n2) {
            let dx = { *n2_pos.get(0).unwrap() - *n1_pos.get(0).unwrap() };
            let dy = { *n2_pos.get(1).unwrap() - *n1_pos.get(1).unwrap() };

            let dist = (dx * dx + dy * dy).sqrt() + *kr;
            let f = dist / *k;
            *n1_speed.get_mut(0).unwrap() += dx * f;
            *n1_speed.get_mut(1).unwrap() += dy * f;
            *n2_speed.get_mut(0).unwrap() -= dx * f;
            *n2_speed.get_mut(1).unwrap() -= dy * f;
        }
    }
}

pub fn apply_attraction_fruchterman_3d(layout: &mut Layout) {
    let k = &layout.settings.ka;
    let kr = &layout.settings.kr;
    for (_edge, (n1, n2)) in layout.edges.iter().enumerate() {
        let n1_pos = layout.points.get(*n1);
        let n2_pos = layout.points.get(*n2);

        if let Some((n1_speed, n2_speed)) = layout.speeds.get_2_mut(*n1, *n2) {
            let dx = { *n2_pos.get(0).unwrap() - *n1_pos.get(0).unwrap() };
            let dy = { *n2_pos.get(1).unwrap() - *n1_pos.get(1).unwrap() };
            let dz = { *n2_pos.get(2).unwrap() - *n1_pos.get(2).unwrap() };

            let dist = (dx * dx + dy * dy + dz * dz).sqrt() + *kr;
            let f = dist / *k;
            *n1_speed.get_mut(0).unwrap() += dx * f;
            *n1_speed.get_mut(1).unwrap() += dy * f;
            *n1_speed.get_mut(2).unwrap() += dz * f;
            *n2_speed.get_mut(0).unwrap() -= dx * f;
            *n2_speed.get_mut(1).unwrap() -= dy * f;
            *n2_speed.get_mut(2).unwrap() -= dz * f;
        }
    }
}

pub fn apply_attraction_forceatlas2_2d(layout: &mut Layout) {
    for (edge, (n1, n2)) in layout.edges.iter().enumerate() {
        let (n1, n2) = (*n1, *n2);

        let n1_pos = layout.points.get(n1);
        let n2_pos = layout.points.get(n2);

        let weight =
            layout.weights.as_ref().map_or(1.0, |weights| weights[edge]) * layout.settings.ka;

        if let Some((n1_speed, n2_speed)) = layout.speeds.get_2_mut(n1, n2) {
            let dx = { *n2_pos.get(0).unwrap() - *n1_pos.get(0).unwrap() } * weight;
            let dy = { *n2_pos.get(1).unwrap() - *n1_pos.get(1).unwrap() } * weight;
            *n1_speed.get_mut(0).unwrap() += dx;
            *n1_speed.get_mut(1).unwrap() += dy;
            *n2_speed.get_mut(0).unwrap() -= dx;
            *n2_speed.get_mut(1).unwrap() -= dy;
        }
    }
}

pub fn apply_attraction_forceatlas2_3d(layout: &mut Layout) {
    for (edge, (n1, n2)) in layout.edges.iter().enumerate() {
        let (n1, n2) = (*n1, *n2);

        let n1_pos = layout.points.get(n1);
        let n2_pos = layout.points.get(n2);
        let weight =
            layout.weights.as_ref().map_or(1.0, |weights| weights[edge]) * layout.settings.ka;

        if let Some((n1_speed, n2_speed)) = layout.speeds.get_2_mut(n1, n2) {
            let dx = { *n2_pos.get(0).unwrap() - *n1_pos.get(0).unwrap() } * weight;
            let dy = { *n2_pos.get(1).unwrap() - *n1_pos.get(1).unwrap() } * weight;
            let dz = { *n2_pos.get(2).unwrap() - *n1_pos.get(2).unwrap() } * weight;

            *n1_speed.get_mut(0).unwrap() += dx;

            *n1_speed.get_mut(1).unwrap() += dy;

            *n1_speed.get_mut(2).unwrap() += dz;

            *n2_speed.get_mut(0).unwrap() -= dx;

            *n2_speed.get_mut(1).unwrap() -= dy;

            *n2_speed.get_mut(2).unwrap() -= dz;
        }
    }
}

pub fn apply_attraction_forceatlas2_dh(layout: &mut Layout) {
    for (edge, (n1, n2)) in layout.edges.iter().enumerate() {
        let f = layout.weights.as_ref().map_or_else(
            || layout.settings.ka.clone(),
            |weights| layout.settings.ka.clone() * weights[edge].clone(),
        );
        let n1_speed = layout.speeds.get_mut(*n1);
        let n1_pos = layout.points.get(*n1);
        let mut di_v = layout.points.get_clone(*n2);
        let di = di_v.as_mut_slice();
        let n1_mass = layout.masses.get(*n1).unwrap().clone();
        for (n1_speed, n1_pos, di) in izip!(n1_speed, n1_pos, di.iter_mut()) {
            *di -= n1_pos.clone();
            *di /= n1_mass.clone();
            *di *= f.clone();
            *n1_speed += di.clone();
        }
        let n2_speed = layout.speeds.get_mut(*n2);
        for i in 0usize..layout.settings.dimensions {
            n2_speed[i] -= di[i].clone();
        }
    }
}

pub fn apply_attraction_forceatlas2_log(layout: &mut Layout) {
    for (edge, (n1, n2)) in layout.edges.iter().enumerate() {
        let mut d = 0.0;
        let mut di_v = layout.points.get_clone(*n2);
        let di = di_v.as_mut_slice();
        for (di, n1) in di.iter_mut().zip(layout.points.get(*n1)) {
            *di -= n1.clone();
            d += di.clone().powi(2);
        }
        if d == 0.0 {
            continue;
        }
        d = d.sqrt();

        let f = d.clone().ln_1p() / d
            * layout.weights.as_ref().map_or_else(
                || layout.settings.ka.clone(),
                |weights| layout.settings.ka.clone() * weights[edge].clone(),
            );

        let n1_speed = layout.speeds.get_mut(*n1);
        for i in 0usize..layout.settings.dimensions {
            n1_speed[i] += f.clone() * di[i].clone();
        }
        let n2_speed = layout.speeds.get_mut(*n2);
        for i in 0usize..layout.settings.dimensions {
            n2_speed[i] -= f.clone() * di[i].clone();
        }
    }
}

pub fn apply_attraction_forceatlas2_dh_log(layout: &mut Layout) {
    for (edge, (n1, n2)) in layout.edges.iter().enumerate() {
        let mut d = 0.0;
        let mut di_v = layout.points.get_clone(*n2);
        let di = di_v.as_mut_slice();
        for (di, n1) in di.iter_mut().zip(layout.points.get(*n1)) {
            *di -= n1.clone();
            d += di.clone().powi(2);
        }
        if d == 0.0 {
            continue;
        }
        d = d.sqrt();

        let n1_mass = layout.masses.get(*n1).unwrap().clone();
        let f = d.clone().ln_1p() / d / n1_mass
            * layout.weights.as_ref().map_or_else(
                || layout.settings.ka.clone(),
                |weights| layout.settings.ka.clone() * weights[edge].clone(),
            );

        let n1_speed = layout.speeds.get_mut(*n1);
        for i in 0usize..layout.settings.dimensions {
            n1_speed[i] += f.clone() * di[i].clone();
        }
        let n2_speed = layout.speeds.get_mut(*n2);
        for i in 0usize..layout.settings.dimensions {
            n2_speed[i] -= f.clone() * di[i].clone();
        }
    }
}

pub fn apply_attraction_forceatlas2_po(layout: &mut Layout) {
    let node_size = &layout.settings.prevent_overlapping.as_ref().unwrap().0;
    for (edge, (n1, n2)) in layout.edges.iter().enumerate() {
        let mut d = 0.0;
        let n1_pos = layout.points.get(*n1);
        let mut di_v = layout.points.get_clone(*n2);
        let di = di_v.as_mut_slice();
        for i in 0usize..layout.settings.dimensions {
            di[i] -= n1_pos[i].clone();
            d += di[i].clone().powi(2);
        }
        d = d.sqrt();

        let dprime = d.clone() - node_size.clone();
        if dprime <= 0.0 {
            continue;
        }
        let f = dprime / d
            * layout.weights.as_ref().map_or_else(
                || layout.settings.ka.clone(),
                |weights| layout.settings.ka.clone() * weights[edge].clone(),
            );

        let n1_speed = layout.speeds.get_mut(*n1);
        for i in 0usize..layout.settings.dimensions {
            n1_speed[i] += f.clone() * di[i].clone();
        }
        let n2_speed = layout.speeds.get_mut(*n2);
        for i in 0usize..layout.settings.dimensions {
            n2_speed[i] -= f.clone() * di[i].clone();
        }
    }
}

pub fn apply_attraction_forceatlas2_dh_po(layout: &mut Layout) {
    let node_size = &layout.settings.prevent_overlapping.as_ref().unwrap().0;
    for (edge, (n1, n2)) in layout.edges.iter().enumerate() {
        let mut d = 0.0;
        let n1_pos = layout.points.get(*n1);
        let mut di_v = layout.points.get_clone(*n2);
        let di = di_v.as_mut_slice();
        for i in 0usize..layout.settings.dimensions {
            di[i] -= n1_pos[i].clone();
            d += di[i].clone().powi(2);
        }
        d = d.sqrt();

        let dprime = d.clone() - node_size.clone();
        if dprime < 0.0 {
            dbg!(dprime);
            continue;
        }
        let n1_mass = layout.masses.get(*n1).unwrap().clone();
        let f = dprime / d / n1_mass
            * layout.weights.as_ref().map_or_else(
                || layout.settings.ka.clone(),
                |weights| layout.settings.ka.clone() * weights[edge].clone(),
            );

        let n1_speed = layout.speeds.get_mut(*n1);
        for i in 0usize..layout.settings.dimensions {
            n1_speed[i] += f.clone() * di[i].clone();
        }
        let n2_speed = layout.speeds.get_mut(*n2);
        for i in 0usize..layout.settings.dimensions {
            n2_speed[i] -= f.clone() * di[i].clone();
        }
    }
}

pub fn apply_attraction_forceatlas2_log_po(layout: &mut Layout) {
    let node_size = &layout.settings.prevent_overlapping.as_ref().unwrap().0;
    for (edge, (n1, n2)) in layout.edges.iter().enumerate() {
        let mut d = 0.0;
        let n1_pos = layout.points.get(*n1);
        let mut di_v = layout.points.get_clone(*n2);
        let di = di_v.as_mut_slice();
        for i in 0usize..layout.settings.dimensions {
            di[i] -= n1_pos[i].clone();
            d += di[i].clone().powi(2);
        }
        d = d.sqrt();

        let dprime = d - node_size.clone();
        if dprime < 0.0 {
            continue;
        }
        let f = dprime.clone().ln_1p() / dprime
            * layout.weights.as_ref().map_or_else(
                || layout.settings.ka.clone(),
                |weights| layout.settings.ka.clone() * weights[edge].clone(),
            );

        let n1_speed = layout.speeds.get_mut(*n1);
        for i in 0usize..layout.settings.dimensions {
            n1_speed[i] += f.clone() * di[i].clone();
        }
        let n2_speed = layout.speeds.get_mut(*n2);
        for i in 0usize..layout.settings.dimensions {
            n2_speed[i] -= f.clone() * di[i].clone();
        }
    }
}

pub fn apply_attraction_forceatlas2_dh_log_po(layout: &mut Layout) {
    let node_size = &layout.settings.prevent_overlapping.as_ref().unwrap().0;
    for (edge, (n1, n2)) in layout.edges.iter().enumerate() {
        let mut d = 0.0;
        let n1_pos = layout.points.get(*n1);
        let mut di_v = layout.points.get_clone(*n2);
        let di = di_v.as_mut_slice();
        for i in 0usize..layout.settings.dimensions {
            di[i] -= n1_pos[i].clone();
            d += di[i].clone().powi(2);
        }
        d = d.sqrt();

        let dprime = d - node_size.clone();
        if dprime < 0.0 {
            continue;
        }
        let n1_mass = layout.masses.get(*n1).unwrap().clone();
        let f = dprime.clone().ln_1p() / dprime / n1_mass
            * layout.weights.as_ref().map_or_else(
                || layout.settings.ka.clone(),
                |weights| layout.settings.ka.clone() * weights[edge].clone(),
            );

        let n1_speed = layout.speeds.get_mut(*n1);
        for i in 0usize..layout.settings.dimensions {
            n1_speed[i] += f.clone() * di[i].clone();
        }
        let n2_speed = layout.speeds.get_mut(*n2);
        for i in 0usize..layout.settings.dimensions {
            n2_speed[i] -= f.clone() * di[i].clone();
        }
    }
}
