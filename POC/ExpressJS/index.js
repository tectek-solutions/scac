const express = require('express');
const bodyParser = require('body-parser');
const { sequelize, Todo } = require('./models/todo');

const app = express();
const PORT = 8000;

app.use(bodyParser.json());

app.post('/todos', async (req, res) => {
    try {
        const { name } = req.body;
        if (!name) {
            return res.status(400).json({ error: 'Title is required' });
        }
        const todo = await Todo.create({ name });
        res.status(201).json(todo);
    } catch (error) {
        res.status(500).json({ error: 'Failed to create To-Do item' });
    }
});

app.get('/todos', async (req, res) => {
    try {
        const todos = await Todo.findAll();
        res.status(200).json(todos);
    } catch (error) {
        res.status(500).json({ error: 'Failed to fetch To-Do items' });
    }
});

app.get('/todos/:id', async (req, res) => {
    try {
        const todo = await Todo.findByPk(req.params.id);
        if (!todo) {
            return res.status(404).json({ error: 'To-Do item not found' });
        }
        res.status(200).json(todo);
    } catch (error) {
        res.status(500).json({ error: 'Failed to fetch To-Do item' });
    }
});

app.put('/todos/:id', async (req, res) => {
    try {
        const { name, description } = req.body;
        const todo = await Todo.findByPk(req.params.id);
        if (!todo) {
            return res.status(404).json({ error: 'To-Do item not found' });
        }
        await todo.update({ name, description });
        res.status(200).json(todo);
    } catch (error) {
        res.status(500).json({ error: 'Failed to update To-Do item' });
    }
});

app.delete('/todos/:id', async (req, res) => {
    try {
        const todo = await Todo.findByPk(req.params.id);
        if (!todo) {
            return res.status(404).json({ error: 'To-Do item not found' });
        }
        await todo.destroy();
        res.status(204).send();
    } catch (error) {
        res.status(500).json({ error: 'Failed to delete To-Do item' });
    }
});

sequelize.sync({ force: true }).then(() => {
    console.log('Database synced');
    app.listen(PORT, () => {
        console.log(`Server is running on http://localhost:${PORT}`);
    });
});
