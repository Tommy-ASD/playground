use plotters::{
    coord::{types::RangedCoordf64, Shift},
    prelude::{
        BitMapBackend, Cartesian2d, ChartBuilder, ChartContext, Circle, CoordTranslate,
        DrawingArea, DrawingBackend, EmptyElement, IntoDrawingArea, LineSeries, PointSeries, Text,
        BLACK, RED, WHITE,
    },
    style::IntoFont,
};

use crate::{Graph, NodeMetaData};

impl Graph {
    pub fn visualize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.visualize_with_path("graph.png")
    }
    pub fn visualize_with_path(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let size = 1000;
        // Create a drawing area
        let root = BitMapBackend::new(path, (size as u32, size as u32)).into_drawing_area();
        root.fill(&WHITE)?;
        self.draw_on_backend(&root)?;
        root.present()?;
        Ok(())
    }
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
            let NodeMetaData { position: (x, y) } = node.meta;
            chart.draw_series(PointSeries::of_element(
                vec![(*x, *y)],
                5,
                &RED,
                &|c, s, st| {
                    return EmptyElement::at(c) // We want the point to be at (x, y)
                        + Circle::new((0, 0), s, st.filled()) // And a circle that is 2 pixels large
                        + Text::new(
                        format!("{}", node.id), // Convert the UUID to a string and display it
                        (0, 0), // Adjust the position to display below the point
                        ("sans-serif", 25.0).into_font(),
                    ); // Add text below the point
                },
            ))?;
        }

        for edge in &self.edges {
            let source_pos = self
                .nodes
                .get(*self.node_lookup.get(&edge.incoming).unwrap())
                .unwrap()
                .meta
                .position;
            let target_pos = self
                .nodes
                .get(*self.node_lookup.get(&edge.outgoing).unwrap())
                .unwrap()
                .meta
                .position;
            let source_pos = (*source_pos.0, *source_pos.1);
            let target_pos = (*target_pos.0, *target_pos.1);
            chart.draw_series(LineSeries::new(vec![source_pos, target_pos], &BLACK))?;
        }
        // Export the plot as an image
        Ok(())
    }
}
