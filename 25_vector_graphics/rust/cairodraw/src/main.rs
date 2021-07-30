fn draw(context: &cairo::Context) -> Result<(), Box<dyn std::error::Error>> {
    let (x, y, x1, y1) = (0.1, 0.5, 0.4, 0.9);
    let (x2, y2, x3, y3) = (0.6, 0.1, 0.9, 0.5);
    context.scale(200., 200.);
    context.set_line_width(0.04);
    context.move_to(x, y);
    context.curve_to(x1, y1, x2, y2, x3, y3);
    context.stroke()?;
    context.set_source_rgba(1., 0.2, 0.2, 0.6);
    context.set_line_width(0.02);
    context.move_to(x, y);
    context.line_to(x1, y1);
    context.move_to(x2, y2);
    context.line_to(x3, y3);
    context.stroke()?;
    Ok(())
}

fn draw_box(
    context: &cairo::Context,
    x: f64,
    y: f64,
    x2: f64,
    y2: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    context.set_source_rgb(0.258, 0.525, 0.956);
    context.new_path();
    context.move_to(x, y);
    context.line_to(x2, y);
    context.line_to(x, y2);
    context.move_to(x2, y2);
    context.line_to(x2, y);
    context.line_to(x, y2);
    context.close_path();
    context.fill()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let png_output_fn = "output.png";
    let img_surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 200, 200).unwrap();
    let svg_fn = std::path::Path::new("output.svg");
    let svg_surface = cairo::SvgSurface::new(200., 200., Some(svg_fn)).unwrap();
    let img_ctx = cairo::Context::new(&img_surface)?;
    let svg_ctx = cairo::Context::new(&svg_surface)?;

    draw_box(&img_ctx, 0., 0., 32., 32.)?;
    draw_box(&svg_ctx, 0., 0., 32., 32.)?;
    draw(&img_ctx)?;
    draw(&svg_ctx)?;
    let mut file = std::fs::File::create(png_output_fn)?;
    img_surface.write_to_png(&mut file).unwrap();
    Ok(())
}
