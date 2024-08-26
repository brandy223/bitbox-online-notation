/* tslint:disable */
/* eslint-disable */
import {Alert} from '../models/alert';

export interface UserConfig {
  alerts: Array<Alert | null>;
  id: number;
  updated_at: string;
  user_id: string;
}
