import os
import sys
import asyncio
import ghsrch
import argparse


TOKEN = os.environ['GITHUB_TOKEN']


async def execute_code_search(query: ghsrch.GithubSearchQuery,
                              target: str,
                              per_page: int):
    try:
        client = ghsrch.GithubClient(TOKEN)

        # We have checked at this point, but for now check again
        if target == 'code':
            results = await client.search_code(query, per_page)
            print(f'Found {results.total_count} matches')
            for result in results.items:
                print(f'{result.repository.full_name}\t{result.name}')
                for match in result.text_matches:
                    print(match.fragment)
                    print("")
                print('======================================================')

        elif target == 'commits':
            results = await client.search_commits(query, per_page)
            print(f'Found {results.total_count} matches')
            for result in results.items:
                print(f'{result.repository.full_name}\t{result.url}')
                for match in result.text_matches:
                    print(match.fragment)
                    print("")
                print('======================================================')
        else:
            raise Exception("Invalid action. Specify either code or commits")

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
    parser.add_argument('target',
                        metavar='T',
                        choices=['code', 'commits'],
                        type=str,
                        help='The target to search. Either `code` or `commits`'
                        )
    parser.add_argument('query',
                        metavar='Q',
                        type=str,
                        nargs='+',
                        help='keywords to search')

    args = parser.parse_args()

    target: str = args.target
    if not target:
        print('Please specify either code or commits to search')
        sys.exit(1)
    target = target.lower()
    if target != 'code' and target != 'commits':
        print('Please specify either code or commits to search')
        sys.exit(1)

    queries = args.query
    if not queries:
        print('Please speficy at least one search term')
        sys.exit(1)

    query_string = ' '.join(queries)
    query = ghsrch.GithubSearchQuery(
        query_string,
        args.user,
        args.filename,
        args.language,
        args.org,
        args.repository
    )

    asyncio.run(
        execute_code_search(query=query, target=target, per_page=args.count)
    )
