async function fetchTasks() {
    const response = await fetch('/tasks');
    const tasks = await response.json();
    const tasksDiv = document.getElementById('tasks');
    tasksDiv.innerHTML = '';
    tasks.forEach((task, index) => {
        tasksDiv.innerHTML += `
            <div class="task">
                <span>${task.description} ${task.completed ? "✓" : "✗"}</span>
                <button onclick="completeTask(${index})">Complete</button>
                <button onclick="deleteTask(${index})">Delete</button>
            </div>
        `;
    });
}

async function addTask() {
    const newTask = document.getElementById('new-task').value;
    await fetch('/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: newTask, completed: false })
    });
    document.getElementById('new-task').value = '';
    fetchTasks();
}

async function completeTask(index) {
    await fetch(`/complete/${index}`, { method: 'POST' });
    fetchTasks();
}

async function deleteTask(index) {
    await fetch(`/delete/${index}`, { method: 'DELETE' });
    fetchTasks();
}

document.addEventListener("DOMContentLoaded", fetchTasks);
