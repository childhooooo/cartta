<template>
  <v-app>
    <v-navigation-drawer
      fixed
      clipped
      class="white"
      id="drawer"
      app
      v-model="drawer"
      v-scroll:#scroll="scroll"
    >
      <v-list id="scroll" three-line>
        <v-text-field
          flat
          solo-inverted
          prepend-icon="search"
          label="Search"
          class="search"
          v-model="input"
          style="padding: 4px 16px"
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
          <v-chip
            small
            label
            disabled
            class="chip jp bold grey lighten-1"
            @click="dialog=true"
          >
            <v-icon color="white">add</v-icon>
          </v-chip>
          <v-layout fixed row justify-center>
            <v-dialog v-model="dialog" max-width="290">
              <v-card>
                <v-card-title class="headline">タグの追加／削除</v-card-title>
                <v-card-text>
                  <v-text-field
                    label="追加するタグ"
                    v-model="to_add"
                    required
                    solo-inverted
                    flat
                  ></v-text-field>
                  <v-select
                    label="削除するタグ"
                    v-model="to_delete"
                    :items="tags"
                    item-text="name"
                    solo-inverted
                    flat
                    return-object
                  ></v-select>
                </v-card-text>
                <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn color="grey darken-3" flat @click.native="dialog = false">キャンセル</v-btn>
                <v-btn color="grey darken-3" flat @click.native="editTag">適用</v-btn>
                <v-spacer></v-spacer>
                </v-card-actions>
              </v-card>
            </v-dialog>
          </v-layout>
        </div>
        <v-divider></v-divider>
        <ListItem
          v-for="note in notes"
          :key="note.listnote.id"
          :title="note.listnote.title"
          :updated_at="note.listnote.updated_at"
          :tags="note.tags"
          :access="note.listnote.access"
          v-on:selected="initNote({ id: note.listnote.id })"
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
        @click="$el.querySelector('#scroll').scrollTop = 0"
      >
        <v-icon>arrow_upward</v-icon>
      </v-btn>
    </v-navigation-drawer>
    <v-toolbar
      color="white"
      app
      absolute
      clipped-left
    >
      <v-toolbar-title style="width: 300px" class="ml-0">
        <v-toolbar-side-icon @click.stop="drawer = !drawer"></v-toolbar-side-icon>
        <a href="/" class="title-toolbar js">Cartta</a>
      </v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn icon @click="newNote">
        <v-icon>add</v-icon>
      </v-btn>
    </v-toolbar>
    <v-content>
      <Editor
        :user="user"
      />
    </v-content>
  </v-app>
</template>

<script>
import ListItem from '../components/ListItem.vue'
import Tag from '../components/Tag.vue'
import Editor from '../components/Editor.vue'
import { mapGetters, mapActions } from 'vuex'
import _ from 'lodash'
import axios from 'axios'
import config from 'config'
export default {
  components: {
    ListItem,
    Tag,
    Editor
  },
  data() {
    return {
      drawer: null,
      input: '',
      dialog: false,
      to_delete: null,
      to_add: '',
      user: {},
    }
  },
  async created() {
    await this.initUser()
    this.setRoute({ route_note: '/note', route_tag: '/tag' })
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
      newNote: 'newNote',
      addTag: 'addTag',
      deleteTag: 'deleteTag',
      selectTag: 'selectTag',
      setRoute: 'setRoute',
      setQuery: 'setQuery'
    }),
    async initUser() {
      let uid = document.getElementById('uid').innerHTML
      let res = await axios.get(config.api_root + '/user/' + uid)
      this.user = res.data
    },
    editTag: function() {
      if(this.to_add != '') {
        this.addTag({ name: this.to_add })
        this.to_add = ''
      }
      if(this.to_delete != null) {
        this.deleteTag({ id: this.to_delete.id })
        this.to_delete = null
      }
      this.dialog = false
    },
    toggleTag: function(select, id) {
      this.selectTag({
        id: id,
        select: select
      })
      this.initNotes()
    },
    scroll: _.throttle(function(e) {
      if ((e.target.scrollTop + e.target.offsetHeight) >= e.target.scrollHeight) {
        this.nextPage()
      }
    }, 500)
  }
}
</script>

<style lang="stylus">
@import 'app'

.v-chip > .v-chip__content
  cursor: pointer !important
  font-size: 13px

.v-chip--small
  height: 22px !important
</style>

<style lang="stylus" scoped>
#app
  height: 100vh
  overflow: hidden

.title-toolbar
  font-size: 32px
  color: #000000
  text-decoration: none

.v-text-field
  padding: 0 15px

.chips
  padding: 0px 15px 5px 15px

.chip
  font-size: 16px
  color: #ffffff
  user-select: none

.v-btn-toggle--selected
  box-shadow: none

.search
  margin-top: 15px

.v-list
  height: 100%
  overflow: scroll
  scroll-behavior: smooth

.v-btn::before
  opacity: 0
</style>