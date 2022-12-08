import { queryDevice } from '@/commands/core'
import { PainterCircle } from '@bindings/PainterCircle';
import { PainterLine } from '@bindings/PainterLine'
import { PainterPixel } from '@bindings/PainterPixel'
import { PainterRect } from '@bindings/PainterRect'
import { PainterEllipse } from '@bindings/PainterEllipse';

export async function drawClear(id: string) {
    const rect: PainterRect = {
        dev: 2,
        left: 0,
        top: 0,
        right: 480,
        bottom: 320,
        color: {
            hue: 0,
            sat: 0,
            val: 0
        },
        filled: 1
    };

    await queryDevice('painter_rect', id, rect)
}

export async function drawPixel(id: string, pixel: PainterPixel) {
    await queryDevice('painter_pixel', id, pixel)
}

export async function drawLine(id: string, line: PainterLine) {
    await queryDevice('painter_line', id, line)
}

export async function drawRect(id: string, rect: PainterRect) {
    await queryDevice('painter_rect', id, rect)
}

export async function drawCircle(id: string, circle: PainterCircle) {
    await queryDevice('painter_circle', id, circle);
}

export async function drawEllipse(id: string, ellipse: PainterEllipse) {
    await queryDevice('painter_ellipse', id, ellipse);
}