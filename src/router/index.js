// src/router/index.js
import Home from '../components/Home.vue';
import About from '../components/About.vue';
import Contact from '../components/Contact.vue';

import { createRouter, createWebHistory } from 'vue-router'
import Setting from '../components/Setting.vue';

const routes = [
    {
        path: '/',
        name: 'Home',
        component: Home
    },
    {
        path: '/About/',
        name: 'About',
        component: About
    },
    {
        path: '/Contact/',
        name: 'Contact',
        component: Contact
    },
    {
        path: '/Setting/',
        name: 'Setting',
        component: Setting
    },
    {
        path: '/img/',
        name: 'img',
        component: ""
    }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router
