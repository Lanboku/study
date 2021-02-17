const localStorageKeyName = "tasks";
const vm = new Vue({
  el: "#app",
  data: {
    newTask: "",
    taskList: [],
  },
  mounted: function () {
    // 初回ロード時に既存のタスクを読み込む
    this.loadTaskList();
  },
  methods: {
    // ローカルストレージに保存したタスクリストを取得
    loadTaskList: function () {
      this.taskList = JSON.parse(localStorage.getItem(localStorageKeyName));
    },
    // タスクリストをローカルストレージに保存
    saveTaskList: function () {
      localStorage.setItem(localStorageKeyName, JSON.stringify(this.taskList));
    },
    // フォームに入力されたタスクをリストに追加
    addTask: function () {
      // フォームが空だった場合は何もしない
      if (this.newTask === "") return;
      this.taskList.push({
        title: this.newTask,
        isChecked: false,
      });

      // タスクの追加後に入力フォームを初期化
      this.newTask = "";
      this.saveTaskList();
    },
  },
});
