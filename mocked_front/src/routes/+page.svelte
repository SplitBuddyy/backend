<script lang="ts">
	import { onMount } from 'svelte';

	let name = '';
	let email = '';
	let password = '';
	let users: string[] = [];
	let groupName = '';
	let groupMembers = '';
	let groups: string[] = [];
	let description = '';
	let amount = '';
	let payerId = '';
	let participants = '';
	let expenses: string[] = [];

	// Interfaces
	export interface GroupSummary {
		group: Group;
		total_spent: number;
		expenses: Expense[];
		transactions: Transaction[];
	}

	export interface Group {
		id?: number;
		name: string;
		owner: number;
		members?: User[];
		expenses?: Expense[];
	}

	export interface User {
		id?: number;
		name: string;
		email: string;
		password: string;
	}

	export interface Expense {
		id: number;
		description?: string;
		amount: number;
		payer: User;
		participants: User[];
		date: string;
	}

	export interface Transaction {
		id: number;
		payer: User;
		receiver: User;
		amount: number;
		date: string;
	}

	// Add User
	async function addUser() {
		const response = await fetch('http://localhost:3000/user/create_user', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ name, email, password })
		});
		const data = await response.text();
		alert(data);
		await fetchUsers();
	}

	// Fetch Users
	async function fetchUsers() {
		const response = await fetch('http://localhost:3000/user/get_users');
		const json = await response.json();
		console.log(json)
		users = json.map((user: User) => `${user.id}: ${user.name} (${user.email})`);
	}

	// Create Group
	async function createGroup() {
		const response = await fetch('http://localhost:3000/group/create_group', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ name: groupName, owner: 1 })
		});
		const data = await response.text();
		alert(data);
		await fetchGroups();
	}

	// Fetch Groups
	async function fetchGroups() {
		const response = await fetch('http://localhost:3000/group/get_groups', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ owner: 1 })
		});
		const json = await response.json();
		groups = json.map((group: Group) => `ID: ${group.id}, Name: ${group.name}`);
	}

	// Add Expense
	async function addExpense() {
		const response = await fetch('http://localhost:3000/group/add_expense', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				group_info: { owner: Number(payerId), group_id: 0 },
				expense: {
					id: Math.floor(Math.random() * 10000),
					description,
					amount: parseFloat(amount),
					payer: { id: Number(payerId), name: "User Name", email: "user@example.com", password: "asdasd" },
					participants: participants.split(',').map(id => ({ id: Number(id), name: "User", email: "user@example.com", password: "sadasd" })),
					date: new Date().toISOString().split('T')[0]
				}
			})
		});
		const data = await response.text();
		alert(data);
		await fetchExpenses();
	}

	// Fetch Expenses
	async function fetchExpenses() {
		const response = await fetch('http://localhost:3000/group/calculate', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ owner: 1, group_id: 0 })
		});
		const data: GroupSummary = await response.json();
		expenses = data.transactions.map(
			(transaction) => `Payer: ${transaction.payer.name} -> Receiver: ${transaction.receiver.name}, Amount: ${transaction.amount}`
		);
	}

	onMount(() => {
		fetchUsers();
		fetchGroups();
		fetchExpenses();
	});
</script>

<!-- ✅ Your Original UI with Styles -->
<section class="container">
	<!-- Add User -->
	<div class="card">
		<h2>Add User</h2>
		<input type="text" bind:value={name} placeholder="Enter user name" />
		<input type="email" bind:value={email} placeholder="Enter email" />
		<input type="password" bind:value={password} placeholder="Enter password" />
		<button on:click={addUser}>Add User</button>
	</div>

	<!-- Users List -->
	<div class="card">
		<h2>Users List</h2>
		<button on:click={fetchUsers}>Refresh Users</button>
		<ul>
			{#each users as user}
				<li>{user}</li>
			{/each}
		</ul>
	</div>

	<!-- Create Group -->
	<div class="card">
		<h2>Create Group</h2>
		<input type="text" bind:value={groupName} placeholder="Enter group name" />
		<button on:click={createGroup}>Create Group</button>
	</div>

	<!-- Groups List -->
	<div class="card">
		<h2>Groups List</h2>
		<button on:click={fetchGroups}>Refresh Groups</button>
		<ul>
			{#each groups as group}
				<li>{group}</li>
			{/each}
		</ul>
	</div>

	<!-- Add Expense -->
	<div class="card">
		<h2>Add Expense</h2>
		<input type="text" bind:value={description} placeholder="Expense description" />
		<input type="number" bind:value={amount} placeholder="Amount" />
		<input type="text" bind:value={payerId} placeholder="Payer ID" />
		<input type="text" bind:value={participants} placeholder="Participants (comma-separated IDs)" />
		<button on:click={addExpense}>Add Expense</button>
	</div>

	<!-- Expenses List -->
	<div class="card">
		<h2>Expenses List</h2>
		<button on:click={fetchExpenses}>Refresh Expenses</button>
		<ul>
			{#each expenses as expense}
				<li>{expense}</li>
			{/each}
		</ul>
	</div>
</section>

<!-- ✅ Your Original CSS -->
<style>
	body {
		font-family: Arial, sans-serif;
		background-color: #f4f4f9;
		margin: 0;
		padding: 0;
		display: flex;
		justify-content: center;
		align-items: center;
		min-height: 100vh;
	}

	.container {
		max-width: 1000px;
		width: 100%;
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 20px;
		padding: 20px;
	}

	.card {
		background: white;
		padding: 20px;
		border-radius: 10px;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 15px;
	}

	h2 {
		font-size: 1.5rem;
		color: #333;
	}

	button {
		padding: 12px 20px;
		background-color: #007bff;
		color: white;
		border: none;
		border-radius: 5px;
		cursor: pointer;
	}

	button:hover {
		background-color: #0056b3;
	}
</style>
