"""
服务器入口文件
直接运行: python -m server
或: python server/__main__.py
"""

import sys
import os

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from unified_server import UnifiedServer


def main():
    import argparse
    parser = argparse.ArgumentParser(description='动物识别专家系统 - 统一服务器')
    parser.add_argument('-p', '--port', type=int, default=8080,
                        help='服务器端口 (默认: 8080)')
    args = parser.parse_args()

    server = UnifiedServer(port=args.port)
    server.start()


if __name__ == '__main__':
    main()
