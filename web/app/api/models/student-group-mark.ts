/* tslint:disable */
/* eslint-disable */
import {Student} from '../models/student';

export interface StudentGroupMark {
  comment?: string | null;
  grader: Student;
  mark?: number | null;
  max_mark: number;
}
