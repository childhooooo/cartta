<template>
  <v-chip
    small
    label
    disabled
    text-color="white"
    class="chip jp bold"
    v-bind:color="selected ? 'grey darken-2' : 'grey lighten-1'"
    @click="clicked"
  >
    <v-icon left color="white">label</v-icon>
    {{ name }}
  </v-chip>
</template>

<script>
export default {
  props: {
    name: {
      type: String
    },
    selected: {
      type: Boolean,
      required: true
    },
    selectable: {
      type: Boolean,
      required: true
    }
  },
  methods: {
    clicked: function() {
      if(!this.selectable) {
        return
      }
      this.$emit('clicked')
    }
  }
}
</script>

<style>
.chips {
  padding: 15px;
}
.chip {
  font-size: 16px;
}
</style>
