<template>
  <v-app>
    <v-content>
      <v-container fluid fill-height>
        <v-layout class="justify-center align-center">
          <v-flex class="xs12 sm8 md4">
            <v-card>
              <v-card-title class="justify-center">
                <h2 class="js">Cartte</h2>
              </v-card-title>
              <v-card-text class="primary-title">
                <v-form>
                  <v-text-field
                    v-model="id"
                    name="id"
                    label="Username or e-mail"
                    type="text"
                  ></v-text-field>
                  <v-text-field
                    v-model="password"
                    name="password"
                    label="Password"
                    type="password"
                  ></v-text-field>
                </v-form>
              </v-card-text>
              <v-card-actions>
                Don't have an account?&nbsp;<a href="/signup">Sign up now</a>
                <v-spacer></v-spacer>
                <v-btn v-on:click="signin" class="flat">Sign in</v-btn>
              </v-card-actions>
            </v-card>
          </v-flex>
        </v-layout>
      </v-container>
      <v-layout fixed row justify-center>
          <v-dialog v-model="dialog_message" max-width="290">
              <v-card>
                  <v-card-text style="text-align: center">
                      {{ message }}
                  </v-card-text>
              </v-card>
          </v-dialog>
      </v-layout>
    </v-content>
  </v-app>
</template>

<script>
import axios from 'axios'
import config from 'config'
export default {
  data: function() {
    return {
      id: '',
      password: '',
      dialog_message: false,
      message: ''
    }
  },
  methods: {
    signin: async function() {
      const data = {
        name: this.id,
        email: this.id,
        password: this.password
      }
      axios
      .post(config.api_root + '/session', data, {
        withCredentials: true
      })
      .then(() => window.location.href = '/')
      .catch(() => this.sendMessage('もう一度お試しください'))
    },
    sendMessage: function(message) {
        this.message = message
        this.dialog_message = true
    },
  }
}
</script>

<style lang="stylus">
@import 'app'
</style>

<style scoped>
h2 {
  font-family: 'Josefin Slab';
  font-weight: 700;
  font-size: 48px;
}
</style>
