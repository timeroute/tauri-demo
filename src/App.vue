<script setup>
import { ref } from 'vue'

const theme = ref('light')
const drawer = ref(true)
const items = ref([
  {
    title: 'Foo',
    value: 'foo',
  },
  {
    title: 'Bar',
    value: 'bar',
  },
  {
    title: 'Fizz',
    value: 'fizz',
  },
  {
    title: 'Buzz',
    value: 'buzz',
  },
])

function onClick () {
  theme.value = theme.value === 'light' ? 'dark' : 'light'
}
</script>

<template>
  <v-app :theme="theme">
    <v-app-bar color="teal-darken-4" image="https://picsum.photos/1920/1080?random">
       <template v-slot:image>
        <v-img
          gradient="to top right, rgba(19,84,122,.8), rgba(128,208,199,.8)"
        ></v-img>
      </template>
      <v-toolbar-title>My files</v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn
        :prepend-icon="theme === 'light' ? 'mdi-weather-sunny' : 'mdi-weather-night'"
        @click="onClick"
      >
        Toggle
      </v-btn>
    </v-app-bar>
    <v-navigation-drawer
      v-model="drawer"
      location="left"
      permanent
    >
      <v-list
        :items="items"
      />
    </v-navigation-drawer>
    <v-main>
      <v-container fluid>
        <v-carousel>
          <v-carousel-item
            src="https://cdn.vuetifyjs.com/images/cards/docks.jpg"
            cover
          ></v-carousel-item>
          <v-carousel-item
            src="https://cdn.vuetifyjs.com/images/cards/hotel.jpg"
            cover
          ></v-carousel-item>
          <v-carousel-item
            src="https://cdn.vuetifyjs.com/images/cards/sunshine.jpg"
            cover
          ></v-carousel-item>
        </v-carousel>
        <v-row dense>
          <v-col
            v-for="n in 4"
            :key="n"
            cols="12"
          >
            <v-card
              :title="`Content ${n}`"
              :subtitle="`Subtitle for Content ${n}`"
              text="Lorem ipsum dolor sit amet consectetur, adipisicing elit.?"
            ></v-card>
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<style>
</style>
