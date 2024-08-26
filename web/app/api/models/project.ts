/* tslint:disable */
/* eslint-disable */
import {ProjectState} from '../models/project-state';

export interface Project {
  description?: string | null;
  end_date: string;
  id: string;
  name: string;
  notation_period_duration: number;
  promotion_id: string;
  start_date: string;
  state: ProjectState;
}
