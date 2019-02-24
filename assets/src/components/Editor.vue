<template>
    <v-container fluid fill-height class="grey lighten-4 pa-0">
        <v-layout row wrap>
            <v-flex d-flex xs12 sm6 class="carte">
                <div class="editor">
                    <textarea :value="input" @input="compile">
                    </textarea>
                </div>
            </v-flex>
            <v-flex d-flex xs12 sm6 class="carte is-right">
                <div class="viewer">
                    <div class="status-lock">
                        <v-btn v-if="note.access === 0" @click="toPublic" flat>
                            <v-icon>lock</v-icon>
                            非公開
                        </v-btn>
                        <v-btn v-else-if="note.access === 2" @click="toProtected" flat>
                            <v-icon>lock_open</v-icon>
                            公開
                        </v-btn>
                        <v-btn v-else @click="toPrivate" flat>
                            <v-icon>link</v-icon>
                            リンクのみ公開
                        </v-btn>
                        <p v-if="note.id !== null">updated: {{ toDateTime(note.updated_at) }}</p>
                        <p v-else>updated: --</p>
                    </div>
                    <div v-if="note.access !== 0" class="shared-url">
                        <a :href="link" target="_blank">{{ link }}</a>
                    </div>
                    <div>
                        <v-select
                            v-model="tags_note"
                            class="tags_select"
                            :items="tags"
                            item-text="name"
                            :menu-props="{
                                    closeOnContentClick: true
                                }"
                            small-chips
                            label="+"
                            multiple
                            solo
                            flat
                            return-object
                        ></v-select>
                    </div>
                    <div class="content-viewer" v-html="compiled"></div>
                    <v-speed-dial
                        v-model="fab"
                        fixed
                        bottom
                        right
                        :direction="'left'"
                        open-on-hover
                        :transition="'slide-y-reverse-transition'"
                    >
                        <v-btn
                            slot="activator"
                            v-model="fab"
                            color="white"
                            fab
                        >
                            <v-icon>menu</v-icon>
                            <v-icon>close</v-icon>
                        </v-btn>
                        <v-btn
                            fab
                            small
                            color="white"
                            @click="save"
                        >
                            <v-icon>done</v-icon>
                        </v-btn>
                        <v-btn
                            fab
                            small
                            color="white"
                            @click="dialog = true"
                        >
                            <v-icon>delete</v-icon>
                        </v-btn>
                        <v-btn
                            fab
                            small
                            color="white"
                            @click="clipLink()"
                        >
                            <v-icon>link</v-icon>
                        </v-btn>
                    </v-speed-dial>
                    <v-layout fixed row justify-center>
                        <v-dialog v-model="dialog" persistent max-width="290">
                            <v-card>
                                <v-card-title class="headline">本当に削除しますか？</v-card-title>
                                <v-card-actions>
                                    <v-spacer></v-spacer>
                                    <v-btn color="grey darken-3" flat @click.native="dialog = false">キャンセル</v-btn>
                                    <v-btn color="grey darken-3" flat @click.native="remove">削除</v-btn>
                                    <v-spacer></v-spacer>
                                </v-card-actions>
                            </v-card>
                        </v-dialog>
                        <v-dialog v-model="dialog_message" max-width="290">
                            <v-card>
                                <v-card-text style="text-align: center">
                                    {{ message }}
                                </v-card-text>
                            </v-card>
                        </v-dialog>
                    </v-layout>
                </div>
            </v-flex>
        </v-layout>
    </v-container>
</template>

<script>
//import Tag from './Tag.vue'
import { mapGetters, mapActions } from 'vuex'
import marked from 'marked'
import _ from 'lodash'
import hljs from 'highlight.js'
import 'highlight.js/styles/agate.css'
import config from 'config'
export default {
    components: {
        //Tag
    },
    props: {
        user: {
            type: Object,
            required: true
        }
    },
    data() {
        return {
            id: null,
            input: '',
            tags_note: [],
            fab: false,
            dialog: false,
            dialog_message: false,
            message: ''
        }
    },
    created() {
        this.init()
    },
    updated() {
        if(this.id != this.note.id) {
            this.init()
        }
    },
    computed: {
        ...mapGetters('note', ['note', 'tags']),
        compiled: function() {
            return marked(this.input)
        },
        link: function() {
            return config.endpoint + '/' + this.user.name + '/' + this.id
        }
    },
    methods: {
        ...mapActions('note', {
            addNote: 'addNote',
            newNote: 'newNote',
            updateNote: 'updateNote',
            deleteNote: 'deleteNote',
            chmodNote: 'chmodNote'
        }),
        init: function() {
            this.id = this.note.id
            this.input = this.note.content
            this.tags_note = this.note.tags
            let renderer = new marked.Renderer()
            renderer.code = function(code) {
                return '<pre><code class="hljs">' + hljs.highlightAuto(code).value + '</code></pre>'
            }
            marked.setOptions({
                headerIds: false,
                sanitize: false,
                renderer: renderer
            })
        },
        reset: function() {
            this.id = null
            this.input = ''
            this.tags_note = []
        },
        save: function() {
            if(this.id === null) {
                this.saveNew()
            } else {
                this.saveOver()
            }
        },
        saveNew: function() {
            const title = this.input.split(/\n/)[0].replace(/#/g, '').replace(/ /, '')
            this.addNote({
                title: title,
                content: this.input,
                tag_ids: this.tags_note.map(t => t.id)
            }).catch(() => {
                this.sendMessage('保存できませんでした')
            })
            this.newNote()
            this.reset()
        },
        saveOver: function() {
            const title = this.input.split(/\n/)[0].replace(/#/g, '').replace(/ /, '')
            this.updateNote({
                id: this.id,
                title: title,
                content: this.input,
                tag_ids: this.tags_note.map(t => t.id)
            }).catch(() => {
                this.sendMessage('保存できませんでした')
            })
        },
        remove: function() {
            if(this.id === null) {
                this.dialog = false
                this.sendMessage('保存されていません')
                return
            }
            this.deleteNote({
                id: this.id
            })
            this.newNote()
            this.dialog = false
        },
        toPrivate: function() {
            this.toggleAccess(0)
        },
        toProtected: function() {
            this.toggleAccess(1)
        },
        toPublic: function() {
            this.toggleAccess(2)
        },
        toggleAccess: function(mode) {
            if(this.id === null) {
                alert('保存されていません')
                return
            }
            this.chmodNote( {
                id: this.id,
                mode: mode
            })
        },
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
        clipLink: function() {
            let temp = document.createElement("textarea")
            temp.value = this.link
            document.body.appendChild(temp)
            temp.select()
            document.execCommand('copy')
            this.sendMessage('Link copied!')
            temp.parentElement.removeChild(temp)
        },
        sendMessage: function(message) {
            this.message = message
            this.dialog_message = true
        },
        compile: _.debounce(function(e) {
            this.input = e.target.value
        }, 300),
    }
}
</script>

<style>
.carte {
    padding: 10px;
}
@media screen and (min-width: 600px) {
    .carte.is-right {
        padding-left: 0;
    }
}
.editor, .viewer {
    width: 100%;
    height: calc(100vh - 84px);
    overflow: scroll;
    max-height: 100%;
    padding: 15px 15px;
    background-color: white;
    text-align: left;
}
.editor textarea {
    height: 100%;
    width: 100%;
    resize: none;
    line-height: 1.7em;
}
.editor textarea:focus {
    outline: none;
}
.content-viewer {
    padding: 8px;
}
.status-lock {
    text-align: left;
    padding: 0 12px;
}
.status-lock .v-btn {
    padding: 0;
    margin: 0;
    margin-right: .9em;
    min-width: 0;
    height: 30px;
    display: inline;
}
.status-lock p {
    display: inline;
    font-size: .9em;
}
.status-lock .v-btn__content {
    font-weight: 600;
    color: #555555;
}
.shared-url {
    padding: 0 12px;
    margin-bottom: .9em;
}
.theme--light.v-chip {
    color: #ffffff;
    background-color: #555555;
}
.v-chip {
    border-radius: 2px !important;
}
.viewer .theme--light.v-chip {
    background-color: #555555;
}
.v-chip--selected {
    box-shadow: none;
    border: none;
}
.tags_select .v-text-field.v-text-field--enclosed .v-input__slot {
    padding: 0;
}
.tags_select .v-input__slot {
    min-height: 0;
    margin-bottom: 0;
}
.tags_select .v-input-controll {
    min-height: 0;
}
.tags_select .v-input__control .v-input__slot .v-select__slot .v-label {
    background-color: #bdbdbd;
    color: #ffffff;
    padding: 0 15px;
    border-radius: 2px;
}
.v-text-field.v-text-field--enclosed .v-text-field__details {
    display: none;
}
.tags_select .v-input__control {
    min-height: 0 !important;
}
.tags_select .chip--select-multi {
    margin: 4px 8px 4px 0;
}
</style>
