import cairo


def draw(context):
    x, y, x1, y1 = 0.1, 0.5, 0.4, 0.9
    x2, y2, x3, y3 = 0.6, 0.1, 0.9, 0.5
    context.scale(200, 200)
    context.set_line_width(0.04)
    context.move_to(x, y)
    context.curve_to(x1, y1, x2, y2, x3, y3)
    context.stroke()
    context.set_source_rgba(1, 0.2, 0.2, 0.6)
    context.set_line_width(0.02)
    context.move_to(x, y)
    context.line_to(x1, y1)
    context.move_to(x2, y2)
    context.line_to(x3, y3)
    context.stroke()


def draw_box(context, x, y, x2, y2):
    context.set_source_rgb(0.258, 0.525, 0.956)
    context.new_path()
    context.move_to(x, y)
    context.line_to(x2, y)
    context.line_to(x, y2)
    context.move_to(x2, y2)
    context.line_to(x2, y)
    context.line_to(x, y2)
    context.close_path()
    context.fill()


if __name__ == "__main__":
    with cairo.SVGSurface("example.svg", 200, 200) as surface:
        context = cairo.Context(surface)
        draw_box(context, 0., 0., 32., 32.)
        draw(context)
    with cairo.ImageSurface(cairo.FORMAT_ARGB32, 200, 200) as surface:
        context = cairo.Context(surface)
        draw_box(context, 0., 0., 32., 32.)
        draw(context)
        surface.write_to_png("output.png")
