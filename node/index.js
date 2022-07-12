import postgres from "postgres";

const startDate = new Date();

const sql = postgres("postgres://localhost:5432/bench");

for (let i = 0; i < 100000; i++) {
  const result = await sql`insert into bench default values`;
}

const endDate = new Date();
console.log(`PT${(endDate.getTime() - startDate.getTime()) / 1000}`);

process.exit(0);
