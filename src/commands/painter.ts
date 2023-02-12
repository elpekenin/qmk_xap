import { queryDevice } from '@/commands/core'
import { PainterCircle } from '@bindings/PainterCircle';
import { PainterDevice } from '@bindings/PainterDevice';
import { PainterGeometry } from '@bindings/PainterGeometry';
import { PainterLine } from '@bindings/PainterLine'
import { PainterPixel } from '@bindings/PainterPixel'
import { PainterRect } from '@bindings/PainterRect'
import { PainterEllipse } from '@bindings/PainterEllipse';

export async function drawClear(id: string, screen_id: PainterDevice) {
    await queryDevice('painter_clear', id, dev)
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

export async function getGeometry(id: string, screen_id: PainterDevice): Promise<PainterGeometry> {
    return await queryDevice('painter_geometry', id, dev);
}