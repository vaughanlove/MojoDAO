import { createWebHistory, createRouter } from "vue-router";
import Client from '@/views/Client.vue'
import About from '@/views/About.vue'
import User from '@/views/User.vue'
import Demo from '@/views/Demo.vue'
import App from "@/App.vue";


export const routes = [
    {
        path: "/client",
        name: "Client",
        component: Client,
    },
    {
        path: "/user",
        name: "User",
        component: User,
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