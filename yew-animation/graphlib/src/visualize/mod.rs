use plotters::{
    coord::{types::RangedCoordf64, Shift},
    prelude::{
        Cartesian2d, ChartBuilder, ChartContext, Circle, DrawingArea, DrawingBackend, EmptyElement,
        LineSeries, PointSeries, Text, BLACK, RED,
    },
    style::IntoFont,
};

use crate::{meta::Coordinate, Graph, NodeMetaData};

impl Graph {
    pub fn draw_on_backend<DB: DrawingBackend>(
        &self,
        root: &DrawingArea<DB, Shift>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        DB::ErrorType: 'static,
    {
        // Create a chart context
        let mut chart = ChartBuilder::on(root)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .margin(5)
            .build_cartesian_2d(-1f64..1f64, -1f64..1f64)?;

        self.draw_on_chart(&mut chart)?;
        // Export the plot as an image
        Ok(())
    }
    pub fn draw_on_chart<DB: DrawingBackend>(
        &self,
        chart: &mut ChartContext<'_, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        DB::ErrorType: 'static,
    {
        // Plot nodes as scatter points
        for node in &self.nodes {
            let NodeMetaData {
                coordinate: Coordinate { x, y },
            } = node.meta;
            chart.draw_series(PointSeries::of_element(
                vec![(*x, *y)],
                4,
                &RED,
                &|c, s, st| {
                    return EmptyElement::at(c) // We want the point to be at (x, y)
                        + Circle::new((0, 0), s, st.filled()) // And a circle that is 2 pixels large
                        + Text::new(
                        format!("{}", node.id), // Convert the UUID to a string and display it
                        (0, 0), // Adjust the position to display below the point
                        ("sans-serif", 15.0).into_font(),
                    ); // Add text below the point
                },
            ))?;
        }

        for edge in &self.edges {
            let source_pos = &self
                .nodes
                .get(*self.node_lookup.get(&edge.incoming).unwrap())
                .unwrap()
                .meta
                .coordinate;
            let target_pos = &self
                .nodes
                .get(*self.node_lookup.get(&edge.outgoing).unwrap())
                .unwrap()
                .meta
                .coordinate;
            let source_pos = (*source_pos.x, *source_pos.y);
            let target_pos = (*target_pos.x, *target_pos.y);
            chart.draw_series(LineSeries::new(vec![source_pos, target_pos], &BLACK))?;
        }
        // Export the plot as an image
        Ok(())
    }
}
