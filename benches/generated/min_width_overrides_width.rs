pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50f32), ..Default::default() },
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch.new_node(stretch::style::Style { ..Default::default() }, &[node0]).unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
}
