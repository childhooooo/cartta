<template>
    <v-list-tile
        class="list-item"
        @click="$emit('selected')">
        <v-list-tile-content>
            <v-list-tile-title class="title-list subtitle">
                <v-icon v-if="access !== undefined" left color="black">{{ status }}</v-icon>
                {{ title }}
            </v-list-tile-title>
            <v-list-tile-sub-title class="caption bold">{{ toDate(updated_at) }}</v-list-tile-sub-title>
            <v-list-tile-sub-title>
                <Tag
                    v-for="tag in tags"
                    :key="tag.id"
                    :name="tag.name"
                    :selected="true"
                    :selectable="false"
                />
            </v-list-tile-sub-title>
        </v-list-tile-content>
    </v-list-tile>
</template>

<script>
import Tag from './Tag.vue'
export default {
    components: {
        Tag
    },
    props: {
        title: {
            type: String,
            required: true
        },
        updated_at: {
            type: String
        },
        tags: {
            type: Array,
            default: function() {
                return []
            }
        },
        access: {
            type: Number
        }
    },
    computed: {
        status: function() {
            switch(this.access) {
                case 2: return 'lock_open'
                case 1: return 'link'
                case 0: return 'lock'
                default: return ''
            }
        },
    },
    methods: {
        toDate: function(timestamp) {
            if(!timestamp) {
                return '--'
            }
            //const date = new Date(timestamp.seconds * 1000)
            const date = new Date(timestamp)
            const year = date.getFullYear()
            const month = date.getMonth() + 1
            const day = date.getDate()
            return [year, month, day].join('.')
        }
    }
}
</script>

<style scoped>
.list-item {
    border-bottom: 1px solid #efefef;
    padding: 5px 0;
}
.chip {
    font-size: 12px;
    margin-left: 0 !important;
    margin-right: 8px !important;
}
.v-chip__content {
    font-size: 12px;
}
.v-chip--small {
}
.v-list__tile__title {
    height: 26px;
    line-height: 26px;
}
.v-list--three-line .v-list__tile__sub-title {
    white-space: nowrap;
}
.v-icon--left {
    margin: 0;
}
</style>
