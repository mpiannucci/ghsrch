import os
import asyncio
import ghsrch


TOKEN = os.environ['GITHUB_PAT']


async def main():
    client = ghsrch.GithubClient(TOKEN)
    results = await client.search_code0('gribberish')
    print(results)


if __name__ == '__main__':
    asyncio.run(main())
