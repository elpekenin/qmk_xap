<script setup lang="ts">
    import { storeToRefs } from 'pinia'
    import { ref, watch, onMounted, onUnmounted } from 'vue'
    import type { Ref } from 'vue'
    import { listen, Event, UnlistenFn } from '@tauri-apps/api/event'

    import { FrontendEvent } from '@bindings/FrontendEvent'
    import { useXAPDeviceStore } from '@/stores/devices'
    import { KeyPosition } from '@bindings/KeyPosition'
    import { KeyPositionConfig } from '@bindings/KeyPositionConfig'
    import { XAPConstants } from '@bindings/XAPConstants'
    import { setKeyCode } from '@/commands/remap'
    import { getKeyMap } from '@/commands/keymap'
    import { notifyError } from '@/utils/utils'
    import { getXapConstants } from '../commands/constants'
    import { XAPKeyInfo } from '@bindings/XAPKeyInfo'

    const store = useXAPDeviceStore()
    const { device } = storeToRefs(store)

    const splitter: Ref<number> = ref(15)
    const keycodeTab: Ref<string> = ref('basic')
    const layerTab: Ref<number> = ref(0)

    const selectedKey: Ref<KeyPosition | null> = ref(null)

    const xapConstants: Ref<XAPConstants | null> = ref(null)

    let unlistenKeyTester: UnlistenFn

    async function set(code: number) {
        if (selectedKey.value) {
            try {
                if (!device.value) {
                    return
                }
                const config: KeyPositionConfig = {
                    layer: selectedKey.value.layer,
                    row: selectedKey.value.row,
                    col: selectedKey.value.col,
                    keycode: code,
                }
                // attempt to set keycode
                await setKeyCode(device.value.id, config)
                // read-back updated keymap - state handling is done in the backend
                device.value.key_info = await getKeyMap(device.value.id)
            } catch (err: unknown) {
                notifyError(err)
            }
        }
    }

    function selectKey(key: XAPKeyInfo) {
        selectedKey.value = { layer: key.position.layer, row: key.position.row, col: key.position.col }
    }

    function colorButton(key: XAPKeyInfo): string {
        if (
            selectedKey.value?.layer == key.position.layer &&
            selectedKey.value?.row == key.position.row &&
            selectedKey.value?.col == key.position.col
        ) {
            return 'grey'
        }
        return 'white'
    }

    function keyLabel(key: XAPKeyInfo): string {
        return key.keycode.label ?? key.keycode.key
    }

    function idForKey(row: number, col:number): string {
        return `row-${row} col-${col}`;
    }

    function buttonId(key: XAPKeyInfo): string {
        return idForKey(key.position.row, key.position.col)
    }

    function _sizeStyle(size: number): string {
        return `${size * 5}rem !important`
    }

    function buttonWidth(key: XAPKeyInfo): string {
        return _sizeStyle(key.coords.w);
    }

    function buttonHeight(key: XAPKeyInfo): string {
        return _sizeStyle(key.coords.h);
    }

    async function drawKey(pressed: boolean, row: number, col: number) {
        const key = document.getElementById(idForKey(row, col));

        if (key === null) {
            return;
        }

        const held_class = "bg-blue";
        const checked_class = "bg-green";

        // cleanup key
        key.classList.remove(held_class, checked_class);
        if (pressed) {
            // draw as held
            key.classList.add(held_class)
        } else {
            // draw as checked
            key.classList.add(checked_class)
        }
    }

    watch(device, async () => {
        selectedKey.value = null
    })

    onMounted(async () => {
        try {
            xapConstants.value = await getXapConstants()
        } catch (err) {
            notifyError(err)
        }

        unlistenKeyTester = await listen(
            'keytester',
            (event: Event<FrontendEvent>) => {
                if (event.payload.kind != 'KeyTester') {
                    return
                }

                const { pressed, row, col } = event.payload.data
                drawKey(pressed, row, col)
            }
        )
    })

    onUnmounted(async () => {
        if (unlistenKeyTester) unlistenKeyTester()
    })
</script>

<template>
    <q-page>
        <!--   Keymap   -->
        <div class="q-pa-md">
            <q-splitter v-model="splitter">
                <template #before>
                    <q-tabs
                        v-model="layerTab"
                        vertical
                        class="text-primary text-center"
                    >
                        <h5>Layer</h5>
                        <!-- eslint-disable-next-line vue/valid-v-for -->
                        <q-tab
                            v-for="(layer, index) in device?.key_info"
                            :name="index"
                            :label="index"
                        />
                    </q-tabs>
                </template>

                <template #after>
                    <q-tab-panels
                        v-model="layerTab"
                        swipeable
                        vertical
                        transition-prev="jump-up"
                        transition-next="jump-up"
                    >
                        <!-- eslint-disable-next-line vue/valid-v-for -->
                        <q-tab-panel
                            v-for="(layer, layer_idx) in device?.key_info"
                            :name="layer_idx"
                        >
                            <!-- eslint-disable-next-line vue/require-v-for-key -->
                            <div
                                v-for="row in layer"
                                class="row q-gutter-x-md q-ma-md"
                            >
                                <!--  TODO create proper Key and Keycode components -->
                                <!-- eslint-disable-next-line vue/valid-v-for -->
                                <q-responsive
                                    v-for="key in row"
                                    class="col"
                                    style="max-width: 3rem"
                                    :ratio="1"
                                >
                                    <div 
                                        v-if="key !== null"
                                        class="btn-size-wrapper"
                                        :width="buttonWidth(key)"
                                        :height="buttonHeight(key)"
                                    >
                                        <q-btn
                                            v-if="key !== null"
                                            :id="buttonId(key)"
                                            block
                                            square
                                            text-color="black"
                                            :color="colorButton(key)"
                                            :label="keyLabel(key)"
                                            @click="() => selectKey(key)"
                                        />
                                    </div>
                                </q-responsive>
                            </div>
                        </q-tab-panel>
                    </q-tab-panels>
                </template>
            </q-splitter>

            <q-separator />

            <!-- Keycodes -->
            <q-splitter v-model="splitter">
                <template #before>
                    <q-tabs
                        v-model="keycodeTab"
                        vertical
                        class="text-primary text-center"
                    >
                        <h5>Keycodes</h5>
                        <!-- eslint-disable vue/no-unused-vars -->
                        <q-tab
                            v-for="category in xapConstants?.keycodes"
                            :key="category.name"
                            :label="category.name"
                            :name="category.name"
                        />
                    </q-tabs>
                </template>

                <template #after>
                    <q-tab-panels
                        v-model="keycodeTab"
                        swipeable
                        vertical
                        transition-prev="jump-up"
                        transition-next="jump-up"
                    >
                        <q-tab-panel
                            v-for="category in xapConstants?.keycodes"
                            :key="category.name"
                            :name="category.name"
                            :label="category.name"
                            class="row q-gutter-md"
                        >
                            <div
                                v-for="code in category.codes"
                                :key="code.code"
                                class="col-1"
                            >
                                <q-responsive
                                    style="max-width: 4rem"
                                    :ratio="1"
                                >
                                    <q-btn
                                        color="white"
                                        :disable="device?.secure_status != 'Unlocked'"
                                        square
                                        text-color="black"
                                        :label="code.label ?? code.key"
                                        @click="set(code.code)"
                                    />
                                    <q-tooltip
                                        v-if="device?.secure_status != 'Unlocked'"
                                        icon="block"
                                        class="bg-red"
                                    >
                                        Device is locked
                                    </q-tooltip>
                                </q-responsive>
                            </div>
                        </q-tab-panel>
                    </q-tab-panels>
                </template>
            </q-splitter>
        </div>
    </q-page>
</template>
