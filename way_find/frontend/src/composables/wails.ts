// @ts-nocheck
import * as WayFindService from './../services/wayfind/backend/wayfindservice.js';

export { MapData, PointData, SearchResultData, StepData, MapInfoData } from './../services/wayfind/backend/index.js';
export type AlgorithmType = 'bfs' | 'dfs' | 'astar';

export const wailsService = {
  ...WayFindService,
};
