const vm = new Vue({
  el: "#app",
  data: {
    newTask: "",
    taskList: [],
  },
  methods: {
    addTask: function () {
      this.taskList.push({
        title: this.newTask,
        isChecked: false,
      });

      this.newTask = "";
    },
  },
});
