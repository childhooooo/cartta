<template>
  <div class="single">
    <!--
  <v-app>
    <div
      class="container single"
    >
    -->
    <!--
      <div class="titlearea">
        <a href="/" class="title-global js">
          anyone's
        </a>
      </div>
      <main>
        <div id="uid" hidden>1</div>
        <div class="status">
          <div class="chips">
            <Tag
              v-for="tag in note.tags"
              :key="tag.id"
              :name="tag.name"
              :selected="true"
              :selectable="false"
            />
          </div>
          <p class="date">{{ toDateTime(note.note.updated_at) }}</p>
        </div>
        <div class="viewer" v-html="compiled"></div>
      </main>
      -->
      <v-divider class="divider"></v-divider>
      <v-list
        three-line id="scrolltop"
        v-scroll="scroll"
      >
          <ListItem
              v-for="note in notes"
              :key="note.listnote.id"
              :title="note.listnote.title"
              :updated_at="note.listnote.updated_at"
              :tags="note.tags"
              v-on:selected="redirect(note.listnote.id)"
          />
      </v-list>
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
    <!--
    </div>
  </v-app>
  -->
  </div>
</template>

<script>
//import Tag from '../components/Tag.vue'
import ListItem from '../components/ListItem.vue'
import { mapGetters, mapActions } from 'vuex'
//import axios from 'axios'
//import config from 'config'
import _ from 'lodash'
//import marked from 'marked'
import hljs from 'highlight.js'
import 'highlight.js/styles/agate.css'
export default {
  components: {
    //Tag,
    ListItem
  },
  data() {
    return {
        user_id: ''
    }
  },
  async created() {
    hljs.initHighlighting();
    this.user_id = document.getElementById('uid').innerHTML;
    this.setRoute({ route_note: '/note/user/' + this.user_id, route_tag: '/tag/user/' + this.user_id })
    this.initNotes()
    /*
    let renderer = new marked.Renderer()
    renderer.code = function(code) {
      return '<pre><code class="hljs">' + hljs.highlightAuto(code).value + '</code></pre>'
    }
    marked.setOptions({
      headerIds: false,
      sanitize: true,
      renderer: renderer
    })
    */
  },
  mounted() {
    setTimeout(() => {
      document.getElementById('loader').classList.add('loaded')
    }, 300)
  },
  computed: {
    ...mapGetters('note', ['notes']),
    /*
    compiled: function() {
      return marked(this.note.note.content)
    }
    */
  },
  methods: {
    ...mapActions('note',{
      setRoute: 'setRoute',
      initNotes: 'initNotes',
      nextPage: 'nextPage',
    }),
    toDateTime: function(timestamp) {
      if(!timestamp) {
          return '--'
      }
      const date = new Date(timestamp)
      const year = date.getFullYear()
      const month = ('0' + (date.getMonth() + 1)).slice(-2)
      const day = ('0' + date.getDate()).slice(-2)
      const hours = ('0' + date.getHours()).slice(-2)
      const minutes = ('0' + date.getMinutes()).slice(-2)
      const res_date = [year, month, day].join('.')
      const res_time = [hours, minutes].join(':')
      return [res_date, res_time].join(' ')
    },
    redirect: function(route) {
      window.location.href = route
    },
    scroll: _.throttle(function() {
      const el = window.document.documentElement
      if (el.scrollHeight - el.clientHeight - el.scrollTop <= 0) {
        this.nextPage()
      }
    }, 500),
  }
}
</script>

<style lang="stylus">
@import 'app'

.application
  background-color: #ffffff !important

.single
  h1
    font-size: 2.4em

  h1:first-child
    margin-top: .4em

  h2
    font-size: 2.1em

  h3
    font-size: 1.7em

  h4
    font-size: 1.4em

  a
    color: #555555

  p, a, code, li, blockquote
    font-size: 1.2em

  blockquote p
    font-size: 1em

  ul li ul li
    font-size: 1em

  p, ul, a
    margin: .8em .4em 1em .4em

  p, a
    margin-left: 0

  blockquote
    margin: 1.2em 0

  code
    margin: 1.2em 0
    font-family: 'Ubuntu Mono' !important
    box-shadow: none

  code:before, code:after
    content: ""

.title-global
  margin: 30px auto
  display: block
  font-weight: 700
  font-size: 80px
  color: #000000
  text-align: center
  text-decoration: none
  cursor: pointer

.v-text-field__details
  display: none !important

.v-chip > .v-chip__content
  cursor: pointer
  font-size: 14px

.v-list__tile__content .caption
  font-size: 16px !important

.single .v-list__tile__title
  height: 30px
  line-height: 30px
  font-size: 20px

.single .list-item
  padding: 15px 0

@media screen and (max-width: 600px)
  .v-chip > .v-chip__content
    font-size: 12px

  .single .v-chip--small
    height: 22px !important

  .v-list__tile__content .caption
    font-size: 13px !important

  .single .v-list__tile__title
    height: 26px
    line-height: 26px
    font-size: 16px

  .single .list-item
    padding: 8px 0

  .title-global
    margin: 16px auto
    font-size: 60px

  .chips
    text-align: left
 
h1, p
  text-align: left

.date
  font-size: 16px

.viewer
  text-align: left

.single .date
  margin: 0
  font-size: 16px

.chips
  padding: 5px 0
  text-align: left

.chip
  color: #ffffff
  user-select: none

.single .v-chip
  margin: 0
  margin-right: 4px

.divider 
    margin-top: 80px

.v-btn::before 
  opacity: 0 !important
</style>