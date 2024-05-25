import { json } from '@sveltejs/kit';

/** @type {import('./$types').RequestHandler} */
export async function POST({ request }) {
	const params = await request.json("params");
	console.log(params)
	return json(params);
}