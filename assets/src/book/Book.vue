<template>
  <div>
    <v-text-field
      flat
      solo-inverted
      prepend-icon="search"
      label="Search"
      class="search"
      v-model="input"
    ></v-text-field>
    <div class="chips">
      <Tag
        v-for="tag in tags"
        :key="tag.id"
        :name="tag.name"
        :selected="tags_selected.indexOf(tag.id) >= 0"
        :selectable="true"
        v-on:clicked="toggleTag(tags_selected.indexOf(tag.id) >= 0 ? false : true, tag.id)"
      />
    </div>
    <v-divider></v-divider>
    <div>
      <v-list three-line class="content" v-scroll="scroll">
        <ListItem
          v-for="note in notes"
          :key="note.listnote.id"
          :title="note.listnote.title"
          :updated_at="note.listnote.updated_at"
          :tags="note.tags"
          v-on:selected="redirect(user.name + '/' + note.listnote.id)"
        />
        <!--
        <v-layout justify-center>
          <v-btn icon flat @click="loadMore(30)">
            <v-icon color="grey lighten-1">cached</v-icon>
            1 - {{ limit }}
          </v-btn>
        </v-layout>
        -->
      </v-list>
    </div>
    <v-btn
      fab
      bottom
      right
      small
      fixed
      color="white"
      class="to-top"
      @click="$vuetify.goTo(0, { duration: 500 })"
    >
      <v-icon>arrow_upward</v-icon>
    </v-btn>
  </div>
</template>

<script>
import ListItem from '../components/ListItem.vue'
import Tag from '../components/Tag.vue'
import { mapGetters, mapActions } from 'vuex'
import _ from 'lodash'
import axios from 'axios'
import config from 'config'
export default {
  components: {
    ListItem,
    Tag
  },
  data() {
    return {
      user: {},
      input: '',
    }
  },
  async created() {
    await this.initUser()
    this.setRoute({ route_note: '/note/user/' + this.user.id , route_tag: '/tag/user/' + this.user.id })
    this.initNotes()
    this.initTags()
  },
  computed: {
    ...mapGetters('note', ['notes', 'tags', 'tags_selected'])
  },
  watch: {
      input: function(query) {
          this.setQuery({ query: query })
          this.initNotes()
      }
  },
  methods: {
    ...mapActions('note', {
      initNotes: 'initNotes',
      nextPage: 'nextPage',
      initTags: 'initTags',
      initNote: 'initNote',
      selectTag: 'selectTag',
      setRoute: 'setRoute',
      setQuery: 'setQuery'
    }),
    async initUser() {
      let uid = document.getElementById('uid').innerHTML;
      let res = await axios.get(config.api_root + '/user/' + uid)
      this.user = res.data
    },
    toggleTag: function(select, id) {
      this.selectTag({
        id: id,
        select: select
      })
      this.initNotes()
    },
    scroll: _.throttle(function() {
      const el = window.document.documentElement
      if (el.scrollHeight - el.clientHeight - el.scrollTop <= 0) {
        this.nextPage()
      }
    }, 500),
    toDate: function(timestamp) {
      if(!timestamp) {
        return '--'
      }
      const date = new Date(timestamp.seconds * 1000)
      const year = date.getFullYear()
      const month = date.getMonth() + 1
      const day = date.getDate()
      return [year, month, day].join('.')
    },
    redirect: function(route) {
      window.location.href = route
    }
  }
}
</script>

<style lang="stylus">
@import 'app'

.v-text-field__details
  display: none !important

.v-chip > .v-chip__content
  cursor: pointer
  font-size: 13px

.v-list__tile__content .caption
  font-size: 16px !important

.book .v-list__tile__title
  height: 30px
  line-height: 30px
  font-size: 20px

.book .list-item
  padding: 15px 0

@media screen and (max-width: 600px)
  .v-chip > .v-chip__content
    font-size: 12px

  .book .v-chip--small
    height: 22px !important

  .v-list__tile__content .caption
    font-size: 13px !important

  .book .v-list__tile__title
    height: 26px
    line-height: 26px
    font-size: 16px

  .book .list-item
    padding: 8px 0

.application
  background-color: #ffffff !important

.container
  padding: 80px 24px
  min-height: 100vh
  max-width: 100vw
  width: 640px
  justify-content: center
  text-align: center

.title-global
  margin: 40px auto
  display: block
  font-weight: 700
  font-size: 80px
  line-height: 1.2em;
  color: #000000

@media screen and (max-width: 600px)
    .container
      padding: 16px 16px
    
    .title-global
      margin: 16px auto
      font-size: 56px
    
    .chips
      text-align: left

.v-text-field
  padding: 0 15px

.chips
  padding: 15px 15px 15px 15px
  text-align: center

.chip
  font-size: 16px
  color: #ffffff
  user-select: none
  margin-left: 0 !important
  margin-right: 8px !important

.v-chip > .v-chip__content
  cursor: pointer !important

.v-btn-toggle--selected
  box-shadow: none

.search
  margin-top: 15px

.v-btn::before
  opacity: 0 !important
</style>
