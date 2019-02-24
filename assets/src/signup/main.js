import Vue from 'vue'
import Vuetify from 'vuetify'
import Signup from './Signup.vue'
import 'vuetify/dist/vuetify.min.css' 

Vue.config.productionTip = false
Vue.use(Vuetify)

new Vue({
  render: h => h(Signup),
}).$mount('#signup')