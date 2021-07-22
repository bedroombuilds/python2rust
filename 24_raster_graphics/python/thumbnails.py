import os, sys
from PIL import Image
import numpy as np


def julia():
    imgx = 800
    imgy = 800

    scalex = 3.0 / imgx
    scaley = 3.0 / imgy

    imgbuf = np.zeros((imgx, imgy, 3), dtype=np.uint8)
    for i in np.ndindex(imgbuf.shape[:2]):
        y, x = i
        r = (0.3 * x)
        b = (0.3 * y)
        cx = float(y) * scalex - 1.5
        cy = float(x) * scaley - 1.5
        c = complex(-0.4, 0.6)
        z = complex(cx, cy)

        g = 0
        while g < 255 and np.hypot(z.real, z.imag) <= 2.0:
            z = z * z + c
            g += 1
        imgbuf[i] = [r, g, b]

    im = Image.fromarray(imgbuf, 'RGB')
    im.save("fractal.png")


if __name__ == "__main__":
    julia()
    for infile in sys.argv[1:]:
        outfile = os.path.splitext(infile)[0] + ".thumbnail.jpg"
        if infile != outfile:
            try:
                with Image.open(infile) as im:
                    im.thumbnail((128, 128))
                    im.save(outfile, "JPEG")
            except OSError:
                print("cannot create thumbnail for", infile)
