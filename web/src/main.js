import { createApp } from 'vue'

const app = createApp({
    data(){
        return {
            msg:"DHT‑Spider 后端服务已就绪",
        }
    },
    template:`
  <div style="padding:24px;font‑family:system‑ui">
    <h2>{{msg}}</h2>
    <p>极简WebUI，后端API正常工作</p>
  </div>
  `
})

app.mount("#app")