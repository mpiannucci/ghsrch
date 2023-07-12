import os
import asyncio
import ghsrch
import argparse


TOKEN = os.environ['GITHUB_PAT']


async def execute_code_search(query: ghsrch.GithubSearchQuery):
    try:
        client = ghsrch.GithubClient(TOKEN)
        results = await client.search_code(query)
        print(f'Found {len(results.items)} matches')
    except Exception as e:
        print(f'Error fetching search')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(
                prog='ghsrchr',
                description='Search github')

    parser.add_argument('term')
    parser.add_argument('-c', '--count')
    parser.add_argument('-u', '--user')
    parser.add_argument('-f', '--filename')
    parser.add_argument('-l', '--language')
    parser.add_argument('-o', '--org')
    parser.add_argument('-r', '--repository')

    args = parser.parse_args()

    query = ghsrch.GithubSearchQuery(
        args.term,
        args.user,
        args.filename,
        args.language,
        args.org,
        args.repository
    )

    print(query.build())

    asyncio.run(execute_code_search(query=query))