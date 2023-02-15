<!-- eslint-disable no-case-declarations -->
<script setup lang="ts">
    import { watch, computed, onMounted, nextTick, ref } from 'vue'
    import type { Ref } from 'vue'
    import { watchPausable } from '@vueuse/core'
    import { storeToRefs } from 'pinia'
    import ColorPicker from '@radial-color-picker/vue-color-picker'

    import { HSVColor } from '@bindings/HSVColor'
    import { PainterDevice } from '@bindings/PainterDevice'
    import { PainterGeometry } from '@bindings/PainterGeometry'
    import { useXAPDeviceStore } from '@/stores/devices'
    import { drawClear, drawCircle, drawLine, drawPixel, drawRect, drawEllipse, getGeometry } from '@/commands/painter'
    import { notifyError } from '@/utils/utils'
    import { createForLoopParams } from '@vue/compiler-core'

    const store = useXAPDeviceStore()
    const { device } = storeToRefs(store)

    const _color: Ref<HSVColor> = ref({
        hue: 255,
        sat: 255,
        val: 255,
    }) 

    const _tool: Ref<string> = ref("Pixel");
    const _filled: Ref<boolean> = ref(false);

    function setCanvasColor() {
        const value = `hsl(${hue.value}, ${_color.value.sat/255*100}%, 50%)`;
        ctx.strokeStyle = value;
        ctx.fillStyle = value;
    }

    const hue = computed({
        get() {
            return Math.ceil((_color.value.hue / 255) * 360)
        },
        set(h: number) {
            _color.value.hue = Math.ceil((h / 360) * 255)
        },
    })

    async function updateHue(h: number) {
        hue.value = h
        setCanvasColor()
    }

    // Setup and react to canvas
    type Coord = {
        x: number,
        y: number,
    }

    type Rect = {
        left: number,
        top: number,
        right: number,
        bottom: number,
    }

    let ctx: CanvasRenderingContext2D;
    let rect: Rect;
    let start: Coord;
    let drawing = false;
    function fromEvent(e: MouseEvent): Coord {
        const value = {
            x: Math.floor(e.clientX - rect.left),
            y: Math.floor(e.clientY - rect.top),
        }
        return value;
    }

    onMounted(async () => {
        const canvas = document.getElementById('painter-canvas') as HTMLCanvasElement;

        if (device?.value == null) {
            return;
        }

        const painter_device: PainterDevice = { id: 0 };
        const geometry: PainterGeometry = await getGeometry(device.value.id, painter_device)

        canvas.style.width = `${geometry.width}px`;
        canvas.style.height = `${geometry.height}px`;
        canvas.width = geometry.width * window.devicePixelRatio;
        canvas.height = geometry.height * window.devicePixelRatio;

        console.log(geometry)

        ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
        rect = canvas.getBoundingClientRect();

        setCanvasColor();

        canvas.addEventListener('mousedown', (e) => {
            drawing = true;
            start = fromEvent(e);
        });

        canvas.addEventListener('mousemove', (e) => {
            if (drawing !== true || _tool.value !== "Pixel") {
                return;
            }

            // Canvas
            const current = fromEvent(e);
            ctx.fillRect(current.x, current.y, 1, 1);

            // QP over XAP
            if (device?.value == null) {
                return;
            }

            drawPixel(
                device.value.id,
                {
                    screen_id: 0,
                    x: current.x,
                    y: current.y,
                    color: _color.value
                }
            ); 
        });

        canvas.addEventListener('mouseup', (e) => {
            if (drawing === false) {
                return;
            }
            
            drawing = false;
            const end = fromEvent(e);

            const x = start.x;
            const y = start.y;
            const x0 = x;
            const y0 = y;
            const x1 = end.x;
            const y1 = end.y;

            const left = Math.min(x0, x1);
            const top = Math.min(y0, y1);
            const right = Math.max(x0, x1);
            const bottom = Math.max(y0, y1);

            const radius = Math.floor(Math.sqrt(Math.pow(end.x-start.x, 2) + Math.pow(end.y-start.y, 2)));

            const sizex = Math.abs(x0-x1);
            const sizey = Math.abs(y0-y1);

            const color = _color.value;
            const filled = _filled.value ? 1 : 0;


            switch (_tool.value){
                case "Line":
                    // Canvas
                    ctx.beginPath();
                    ctx.moveTo(start.x, start.y);
                    ctx.lineTo(end.x, end.y);
                    ctx.stroke();

                    // QP over XAP
                    if (device?.value == null) {
                        return;
                    }

                    drawLine(
                        device.value.id,
                        {
                            screen_id: 0,
                            x0,
                            y0,
                            x1,
                            y1,
                            color,
                        }
                    );
                    break;

                case "Rect":
                    // Canvas
                    if (filled) {
                        ctx.fillRect(left, top, right-left, bottom-top);
                    } else {
                        ctx.beginPath();
                        ctx.moveTo(left, top);
                        ctx.lineTo(right, top);
                        ctx.lineTo(right, bottom);
                        ctx.lineTo(left, bottom);
                        ctx.lineTo(left, top);
                        ctx.stroke();
                    }

                    // QP over XAP
                    if (device?.value == null) {
                        return;
                    }

                    drawRect(
                        device.value.id,
                        {
                            screen_id: 0,
                            left,
                            top,
                            right,
                            bottom,
                            color,
                            filled,
                        }
                    );
                    break;

                case 'Circle':
                    // Canvas
                    ctx.beginPath();
                    ctx.arc(x, y, radius, 0, 2*Math.PI);
                    if (filled) {
                        ctx.fill()
                    } else {
                        ctx.stroke();
                    }

                    // QP over XAP
                    if (device?.value == null) {
                        return;
                    }
                    
                    drawCircle(
                        device.value.id,
                        {
                            screen_id: 0,
                            x,
                            y,
                            radius,
                            color,
                            filled,

                        }
                    )

                    break;

                case 'Ellipse':
                    // Canvas
                    ctx.beginPath();
                    ctx.ellipse(x, y, sizex, sizey, 0, 0, 2*Math.PI);
                    if (filled) {
                        ctx.fill()
                    } else {
                        ctx.stroke();
                    }

                    // QP over XAP
                    if (device?.value == null) {
                        return;
                    }
                    
                    drawEllipse(
                        device.value.id,
                        {
                            screen_id: 0,
                            x,
                            y,
                            sizex,
                            sizey,
                            color,
                            filled,
                        }
                    )

                    break;
            }
       });
    })

    async function clear() {
        ctx.fillStyle = "hsl(0, 0%, 100%)";
        ctx.clearRect(0, 0, rect.right-rect.left, rect.bottom-rect.top);
        setCanvasColor();
        
        if (device?.value == null) {
                return;
        }

        const painter_device: PainterDevice = { id: 0 };
        await drawClear(device.value.id, painter_device);
    }
</script>

<template>
    <q-page>
        <div style="padding-bottom: 30vh;" />

        <div class="row flex-center">
            <div style="margin-right: 100px;">
                <canvas id="painter-canvas" />
            </div>
            
            <div class="column flex-center">
                <div class="row">
                    <color-picker
                        :hue="hue"
                        style="width: 100px; height: 100px; margin-right: 30px;"
                        @input="updateHue"
                    />

                    <q-select
                        v-model.string.lazy="_tool"
                        rounded
                        outlined
                        :options="['Circle', 'Ellipse', 'Line', 'Pixel', 'Rect']"
                        label="Tool"
                        emit-value
                        style="margin-top: auto; margin-bottom: auto;"
                    />
                </div>
                <br>
                <div class="row">
                    <q-btn
                        color="white"
                        text-color="black"
                        label="Clear"
                        @click="clear"
                    />
                    <q-checkbox
                        v-if="['Circle', 'Ellipse', 'Rect'].includes(_tool)"
                        v-model.boolean.lazy="_filled"
                        label="Filled"
                    />
                </div>
            </div>
        </div>
    </q-page>
</template>

<style>
    @import '@radial-color-picker/vue-color-picker/dist/vue-color-picker.min.css';

    canvas {
        border: 5px solid black;
    }
</style>
