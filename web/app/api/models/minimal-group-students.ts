/* tslint:disable */
/* eslint-disable */
import {MinimalStudent} from '../models/minimal-student';

export interface MinimalGroupStudents {
  group_id: string;
  students: Array<MinimalStudent>;
}
