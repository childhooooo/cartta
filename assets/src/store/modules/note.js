import axios from 'axios'
import config from 'config'

export const strict = false

const API_ROOT = config.api_root
const PER_PAGE = config.per_page

const state = {
  notes: [],
  tags: [],
  note: {
    id: null,
    title: '',
    content: '',
    updated_at: '',
    access: 0,
    tags: []
  },
  route_note: '/note',
  route_tag: '/tag',
  query: '',
  tags_selected: [],
  page: 1,
  error: null
}

const getters = {
  notes: state => state.notes,
  tags: state => state.tags,
  note: state => state.note,
  error: state => state.error,
  route_note: state => state.route_note,
  route_tag: state => state.route_tag,
  tags_selected: state => state.tags_selected,
  query: state => state.query
}

const mutations = {
  setNotes (state, notes) {
    state.notes = notes
  },
  setTags (state, tags) {
    state.tags = tags
  },
  pushTag (state, tag) {
    state.tags.push(tag)
  },
  setNote (state, note) {
    state.note = note
  },
  setAccess (state, access) {
    state.note.access = access
  },
  setError (state, error) {
    state.error = error
  },
  pageNext (state) {
    ++state.page;
  },
  resetPage (state) {
    state.page = 1
  },
  setNoteRoute (state, route) {
    state.route_note = route
  },
  setTagRoute (state, route) {
    state.route_tag = route
  },
  selectTag (state, id) {
    state.tags_selected.push(id)
  },
  excludeTag (state, id) {
    state.tags_selected = state.tags_selected.filter(t => t != id)
  },
  setQuery (state, query) {
    state.query = query
  }
}

const actions = {
  initNotes ({ commit, dispatch }) {
    commit('resetPage')
    dispatch('loadNotes')
  },
  nextPage ({ commit, dispatch }) {
    commit('pageNext')
    dispatch('loadNotes')
  },
  loadNotes ({ commit }) {
    let route = format('{0}?page={1}&per_page={2}', state.route_note, state.page, PER_PAGE)
    if(state.query) { route = route + '&query=' + state.query }
    if(state.tags_selected.length) { route = route + '&tag=' + state.tags_selected.reduce((tag_ids, id) => tag_ids + ',' + String(id)) }
    if(state.page === 1) { commit('setNotes', []) }
    axios
    .get(url(route), { withCredentials: true })
    .then(res => commit('setNotes', state.notes.concat(res.data)))
    .catch(err => console.log(err))
  },
  initNote ({ commit }, data) {
    let route = format('/note/{0}', data.id)
    axios
    .get(url(route), { withCredentials: true })
    .then(res => commit('setNote', fetch_note(res.data)))
    .catch(err => console.log(err))
  },
  newNote ({ commit }) {
    const note = {
      id: null,
      title: '',
      content: '',
      updated_at: '',
      access: 0,
      tags: []
    }
    commit('setNote', note)
  },
  addNote ({ dispatch }, data) {
    const u = url('/note')
    console.log(gen_note(data))
    axios
    .post(u, gen_note(data), { withCredentials: true })
    .then(() => dispatch('initNotes', { route: '/note' }))
    .catch(err => console.log(err))
  },
  updateNote ({ dispatch }, data) {
    const u = url(format('/note/{0}', data.id))
    axios
    .put(u, gen_note(data), { withCredentials: true })
    .then(() => dispatch('initNotes', { route: '/note' }))
    .catch(err => console.log(err))
  },
  chmodNote ({ commit, dispatch }, data) {
    const u = url(format('/note/{0}/access?mode={1}', data.id, data.mode))
    axios
    .put(u, { withCredentials: true }, { withCredentials: true })
    .then(res => {
      commit('setAccess', res.data.access)
      dispatch('initNotes')
    })
    .catch(err => console.log(err))
  },
  async deleteNote ({ dispatch }, data) {
    const u = url(format('/note/{0}', data.id))
    await axios
    .delete(u, { withCredentials: true })
    .catch(err => console.log(err))
    dispatch('newNote')
    dispatch('initNotes')
  },
  initTags ({ commit }) {
    axios
    .get(url(state.route_tag), { withCredentials: true })
    .then(res => commit('setTags', res.data))
    .catch(err => console.log(err))
  },
  addTag ({ commit }, data) {
    axios
    .post(url('/tag'), { name: data.name }, { withCredentials: true })
    .then(res => commit('setTags', state.tags.concat(res.data)))
    .catch(err => console.log(err))
  },
  async deleteTag ({ dispatch }, data) {
    await axios
    .delete(url(format('/tag/{0}', data.id)), { withCredentials: true })
    .catch(err => { console.log(err)})
    dispatch('initTags')
  },
  setRoute ({ commit }, data) {
    commit('setNoteRoute', data.route_note)
    commit('setTagRoute', data.route_tag)
  },
  selectTag ({ commit }, data) {
    if(data.select) {
      commit('selectTag', data.id)
    } else {
      commit('excludeTag', data.id)
    }
  },
  setQuery ({ commit }, data) {
    commit('setQuery', data.query)
  }
}

function url (route) {
  return API_ROOT + route
}

function format (str) {
  return str.replace(/\{(\w+)\}/g, (_m, k) => { return String(arguments[parseInt(k)+1]) })
}

function fetch_note (data) {
  return {
    id: data.note.id,
    title: data.note.title,
    content: data.note.content,
    updated_at: data.note.updated_at,
    access: data.note.access,
    tags: data.tags
  }
}

function gen_note (data) {
  return {
    title: data.title,
    content: data.content,
    tag_ids: data.tag_ids
  }
}

export default {
  namespaced: true,
  state,
  getters,
  actions,
  mutations
}