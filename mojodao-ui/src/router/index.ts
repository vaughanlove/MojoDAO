import { createWebHistory, createRouter } from "vue-router";
import Client from '@/views/Client.vue'
import About from '@/views/About.vue'
import Token from '@/views/Token.vue'
import Demo from '@/views/Demo.vue'


export const routes = [
    {
        path: "/client",
        name: "Client",
        component: Client,
    },
    {
        path: "/token",
        name: "Token",
        component: Token,
    },
    {
        path: "/about",
        name: "About",
        component: About,
    },
    {
        path: "/demo",
        name: "Demo",
        component: Demo,
    },
];

export const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;