import Vue from 'vue'
import Vuetify from 'vuetify'
import Signin from './Signin.vue'
import 'vuetify/dist/vuetify.min.css' 

Vue.config.productionTip = false
Vue.use(Vuetify)

new Vue({
  render: h => h(Signin),
}).$mount('#signin')