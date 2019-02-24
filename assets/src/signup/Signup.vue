<template>
  <v-app>
    <v-content>
      <v-container fluid fill-height>
        <v-layout class="justify-center align-center">
          <v-flex class="xs12 sm8 md4">
            <v-card>
              <v-card-title class="justify-center">
                <h2>Cartte</h2>
              </v-card-title>
              <v-card-text class="primary-title">
                <v-form>
                  <v-text-field
                    v-model="username"
                    name="username"
                    label="Username"
                    :rules="rules_name"
                    :counter="20"
                  ></v-text-field>
                  <v-text-field
                    v-model="email"
                    name="email"
                    label="E-mail"
                    :rules="rules_email"
                  ></v-text-field>
                  <v-text-field
                    v-model="password"
                    name="password"
                    label="Password"
                    type="password"
                    :rules="rules_password"
                  ></v-text-field>
                  <v-text-field
                    v-model="password_again"
                    name="password_again"
                    label="Confirm password"
                    type="password"
                    :rules="rules_password_again"
                  ></v-text-field>
                </v-form>
              </v-card-text>
              <v-card-actions>
                Already have an account?&nbsp;<a href="/signin">Sign in</a>
                <v-spacer></v-spacer>
                <v-btn v-on:click="signup" class="flat">Sign up</v-btn>
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
      username: '',
      email: '',
      password: '',
      password_again: '',
      dialog_message: false,
      message: '',
      rules_name: [
        v => !!v || '入力されていません',
        v => v.length <= 20 || '20文字以内で入力してください',
        v => /^[a-z1-9-]*$/.test(v) || '使用できない文字が含まれています'
      ],
      rules_email: [
        v => !!v || '入力されていません',
        v => /.+@.+/.test(v) || 'メールアドレスが正しくありません'
      ],
      rules_password: [
        v => !!v || '入力されていません',
        v => v.length <= 100 || '100文字以内で入力してください',
        v => v.length >= 8 || '8文字以上で入力してください'
      ],
      rules_password_again: [
        v => !!v || '入力されていません',
        v => v === this.password || 'パスワードが違います'
      ],
    }
  },
  computed: {
  },
  methods: {
    signup: function() {
      const data = {
        name: this.username,
        email: this.email,
        password: this.password
      }
      axios
      .post(config.api_root + '/user', data)
      .then(() => window.location.href = '/signin')
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