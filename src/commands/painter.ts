import { queryDevice } from '@/commands/core'
import { PainterCircle } from '@bindings/PainterCircle';
import { PainterGeometry } from '@bindings/PainterGeometry';
import { PainterLine } from '@bindings/PainterLine'
import { PainterPixel } from '@bindings/PainterPixel'
import { PainterRect } from '@bindings/PainterRect'
import { PainterEllipse } from '@bindings/PainterEllipse';

export async function drawClear(id: string, screen: number) {
    // clear command -- on some devices this does nothing
    await queryDevice('painter_clear', id, screen)

    // fill screen black
    const geometry = await getGeometry(id, screen);
    await drawRect(
        id,
        {
            screen_id: screen,
            left: 0,
            top: 0,
            right: geometry.width,
            bottom: geometry.height,
            color: {
                hue: 0,
                sat: 0,
                val: 0
            },
            filled: 1
        }
    );
}

export async function drawPixel(id: string, pixel: PainterPixel) {
    await queryDevice('painter_pixel', id, pixel);
}

export async function drawLine(id: string, line: PainterLine) {
    await queryDevice('painter_line', id, line);
}

export async function drawRect(id: string, rect: PainterRect) {
    await queryDevice('painter_rect', id, rect);
}

export async function drawCircle(id: string, circle: PainterCircle) {
    await queryDevice('painter_circle', id, circle);
}

export async function drawEllipse(id: string, ellipse: PainterEllipse) {
    await queryDevice('painter_ellipse', id, ellipse);
}

export async function getGeometry(id: string, screen: number): Promise<PainterGeometry> {
    return await queryDevice('painter_geometry', id, screen);
}