<script setup>
import { reactive, toRaw, toRef } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import actionType from "./assets/action-type.json"
const actionList = reactive([])
const addAction = () => {
    actionList.push(JSON.parse(JSON.stringify((actionType.move_mouse))));
}
const deleteAction = (key) => {
    actionList.splice(key, 1)
}

const runAction = async () => {
    for (const element of actionList) {
        console.log(element.action_type)
        switch (element.action_type) {
            case "move_mouse":
                debugger
                await invoke("move_mouse", {
                    x: parseInt(element.arg.x), y: parseInt(element.arg.y)
                })
                break;
            case "click_mouse":
                await invoke("click_mouse")
                break;
            case "key_sequence":
                await invoke("key_sequence", {
                    value: element.arg.value
                })
                break;
        }
    }
}

const changeType = (key, event) => {
    actionList[key] = actionType[event.target.value]
}



</script>

<template>
    <div class="container">
        <div class="row">
            <h1>Automatic Mouse</h1>
            <p class="author">By Nightscratch（Nights）</p>
        </div>
        <hr>
        <h3 style="margin: 0;margin-bottom: 5px;">操作</h3>
        <ul>
            <li class="action-card" v-for="(action, action_index) in actionList">
                <p> <button style="padding: 2px;background-color: rgb(239, 50, 50);color: wheat;"
                        @click="deleteAction(action_index)">删除</button>
                    <select @change="changeType(action_index, $event)">
                        <option value="move_mouse">移动鼠标</option>
                        <option value="click_mouse">点击鼠标</option>
                        <option value="key_sequence">输入</option>
                    </select>
                </p>

                <div v-for="(value, key) in action.arg" class="row" style="margin-top: 5px;margin-left: 5px;">
                    <p style="margin-right: 5px;">{{ key }}：</p>
                    <input type="text" class="action-arg" v-model="actionList[action_index].arg[key]">
                </div>

            </li>

            <p v-if="actionList.length == 0">请点击下面添加操作</p>
        </ul>
        <hr>
        <button @click="addAction">添加操作</button>
        <button @click="runAction" style="margin-left: 10px;" v-show="actionList.length != 0">执行</button>
    </div>
</template>

<style scoped>
.author {
    position: absolute;
    margin-top: 5px;
    left: 280px;
    /* 反正没人看源码 */
}

select {
    margin-left: 10px;
    border: #396cd8 solid 2px;
}

select:active {
    border: #396cd8 solid 2px;
}

.action-card {
    margin-top: 5px;
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 5px;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

ul {
    list-style-type: none;
    padding: 0;
    margin: 0;
}

.action-arg {
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    border: #396cd8 solid 2px;
    border-radius: 5px;
}
</style>
