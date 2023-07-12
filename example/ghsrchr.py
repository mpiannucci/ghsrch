import os
import asyncio
import ghsrch
import argparse


TOKEN = os.environ['GITHUB_TOKEN']


async def execute_code_search(query: ghsrch.GithubSearchQuery, per_page: int):
    try:
        client = ghsrch.GithubClient(TOKEN)
        results = await client.search_code(query, per_page)
        print(f'Found {results.total_count} matches')
        for result in results.items:
            print(f'{result.name}\t{result.repository.full_name}')
            for match in result.text_matches:
                print(match.fragment)
                print("")
            print('----------------------------------------------------------')
    except Exception as e:
        print(f'Error fetching search: {e}')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(
                prog='ghsrchr',
                description='Search github')

    parser.add_argument('-c', '--count',
                        default=30,
                        type=int,
                        help='limit search to number of results. Max is 100')
    parser.add_argument('-u', '--user',
                        help='limit search to given username')
    parser.add_argument('-f', '--filename',
                        help='limit search to given filenames')
    parser.add_argument('-l', '--language',
                        help='limit search to limited language')
    parser.add_argument('-o', '--org', help='limit search to specified org')
    parser.add_argument('-r', '--repository',
                        help='limit search to specified repository')
    parser.add_argument('query',
                        metavar='Q',
                        type=str,
                        nargs='+',
                        help='keywords to search')

    args = parser.parse_args()

    queries = args.query
    if not queries:
        print('Please speficy at least one search term')
    query = ' '.join(queries)

    query = ghsrch.GithubSearchQuery(
        query,
        args.user,
        args.filename,
        args.language,
        args.org,
        args.repository
    )

    asyncio.run(execute_code_search(query=query, per_page=args.count))
