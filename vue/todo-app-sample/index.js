const vm = new Vue({
  el: "#app",
  data: {
    newTask: "",
    taskList: [],
  },
  mounted: function () {
    this.loadTaskList();
  },
  methods: {
    loadTaskList: function () {
      this.taskList = JSON.parse(localStorage.getItem("tasks"));
    },
    saveTaskList: function () {
      localStorage.setItem("tasks", JSON.stringify(this.taskList));
    },
    addTask: function () {
      this.taskList.push({
        title: this.newTask,
        isChecked: false,
      });

      this.newTask = "";
      this.saveTaskList();
    },
  },
});
