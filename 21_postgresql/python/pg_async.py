import asyncio
import psycopg2.extras
import aiopg
import helper


async def go():
    async with aiopg.create_pool("host=localhost user=htd") as pool:
        async with pool.acquire() as conn:
            async with conn.cursor(cursor_factory=psycopg2.extras.DictCursor) as cur:
                await cur.execute(helper.MOST_RECOVERED)
                rows = await cur.fetchall()
                print('most recoveries', dict(rows[0]))

                await cur.execute("BEGIN")  # conn.commit() not available
                await cur.execute("DELETE FROM month_cases")
                await cur.execute(helper.GROUP_BY_MONTH)
                ret = []
                async for row in cur:
                    ret.append(row)
                for row in ret:
                    print(row)
                    await cur.execute(
                        '''INSERT INTO month_cases
                        (mon, new_cases, recovered)
                        VALUES (%s, %s, %s)''', row)
                await cur.execute("COMMIT")  # conn.commit() not available


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(go())
