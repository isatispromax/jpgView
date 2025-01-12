<template>
    <div class="side-menu-container" @mouseover="showMenu" @mouseleave="hideMenu">
        <div class="side-menu" :class="{ 'menu-visible': isMenuVisible }">
            <ul>
                <li><a href="/">Home</a></li>
                <li><a href="/About">About</a></li>
                <li><a href="/Setting">Setting</a></li>
                <li><a href="/Contact">Contact</a></li>
            </ul>
        </div>
    </div>
</template>

<script>
export default {
    data() {
        return {
            isMenuVisible: false
        };
    },
    methods: {
        showMenu() {
            this.isMenuVisible = true;
        },
        hideMenu() {
            this.isMenuVisible = false;
        }
    }
};
</script>

<style>
.side-menu-container {
    width: 50px;
    /* Width of the hidden menu */
    background-color: transparent;
    position: fixed;
    top: 0;
    left: 0;
    bottom: 0;
    transition: width 0.6s ease;
}

.side-menu {
    width: 200px;
    /* Width of the expanded menu */
    background-color: #555;
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    overflow: hidden;
    transition: opacity 0.6s ease, visibility 0.6s ease;
    opacity: 0;
    visibility: hidden;
}

.side-menu.menu-visible {
    opacity: 1;
    visibility: visible;
}

.side-menu ul {
    list-style: none;
    padding: 0;
    margin: 0;
}

.side-menu li a {
    display: block;
    padding: 10px;
    color: white;
    text-decoration: none;
}

.side-menu li a:hover {
    background-color: #777;
}

.side-menu-container:hover {
    width: 250px;
    /* Width of the container when menu is visible */
}
</style>