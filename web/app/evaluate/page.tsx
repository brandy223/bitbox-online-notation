'use client'

import React, {useEffect, useState} from "react";
import {MinimalGroupStudents} from "@/app/api/models/minimal-group-students";
import {GradedStudentPostModel} from "@/app/api/models/graded-student-post-model";

async function getPeopleToMark(): Promise<MinimalGroupStudents> {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${api_url}/marks/group-to-evaluate`, {
        headers: { "Content-Type": "application/json" },
        credentials: "include",
    });
    const data = await response.json();
    return data as MinimalGroupStudents;
}

async function submitGrades(grades: GradedStudentPostModel[], group_id: string): Promise<void> {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    await fetch(`${api_url}/marks/evaluate/group/${group_id}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify(grades),
    });
}

const EvaluationPage: React.FC = () => {
    const [minimalGroupStudents, setMinimalGroupStudents] = useState<MinimalGroupStudents>({
        group_id: "",
        students: [],
    });
    const [grades, setGrades] = useState<GradedStudentPostModel[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [submitError, setSubmitError] = useState<string | null>(null);
    const [submitSuccess, setSubmitSuccess] = useState<boolean>(false);

    useEffect(() => {
        const fetchPeopleToMark = async () => {
            setLoading(true);
            setError(null);
            try {
                const groupStudents = await getPeopleToMark();
                setMinimalGroupStudents(groupStudents);
                // Initialize grades with empty values
                const initialGrades = groupStudents.students.map(student => ({
                    student_id: student.student_id,
                    mark: 0,
                    comment: "",
                }));
                setGrades(initialGrades);
            } catch (err) {
                setError("An error occurred. Please try again later.");
            } finally {
                setLoading(false);
            }
        }

        fetchPeopleToMark();
    }, []);

    const handleMarkChange = (student_id: string, mark: number) => {
        setGrades(prevGrades =>
            prevGrades.map(grade =>
                grade.student_id === student_id ? { ...grade, mark } : grade
            )
        );
    };

    const handleCommentChange = (student_id: string, comment: string) => {
        setGrades(prevGrades =>
            prevGrades.map(grade =>
                grade.student_id === student_id ? { ...grade, comment } : grade
            )
        );
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setSubmitError(null);
        setSubmitSuccess(false);
        try {
            await submitGrades(grades, minimalGroupStudents.group_id);
            setSubmitSuccess(true);
        } catch (err) {
            setSubmitError("An error occurred during submission. Please try again later.");
        }
    };

    return (
        <div className="min-h-screen bg-gray-100 p-8">
            <h1 className="text-3xl font-bold text-gray-800 mb-8">People to Mark</h1>
            {loading ? (
                <p className="text-center text-gray-600">Loading...</p>
            ) : error ? (
                <p className="text-center text-red-600">{error}</p>
            ) : (
                <form onSubmit={handleSubmit} className="space-y-6">
                    {minimalGroupStudents.students.map((student, index) => (
                        <div key={student.student_id} className="bg-white p-6 rounded-lg shadow-md">
                            <label className="block text-lg font-medium text-gray-700 mb-4">
                                {student.name} {student.surname}
                            </label>
                            <div className="flex items-center space-x-4">
                                <input
                                    type="number"
                                    value={grades[index]?.mark || 0}
                                    onChange={(e) => handleMarkChange(student.student_id, parseInt(e.target.value))}
                                    min="0"
                                    max="20"
                                    required
                                    className="w-20 p-2 border border-gray-300 rounded-md text-gray-800 bg-gray-100"
                                />
                                <input
                                    type="text"
                                    placeholder="Add a comment (optional)"
                                    value={grades[index]?.comment || ""}
                                    onChange={(e) => handleCommentChange(student.student_id, e.target.value)}
                                    className="flex-grow p-2 border border-gray-300 rounded-md text-gray-800 bg-gray-100"
                                />
                            </div>
                        </div>
                    ))}
                    <div className="flex justify-end">
                        <button
                            type="submit"
                            className="bg-blue-600 text-white py-2 px-4 rounded-lg shadow hover:bg-blue-500 transition-all"
                        >
                            Submit Grades
                        </button>
                    </div>
                </form>
            )}
            {submitSuccess && <p className="text-center text-green-600 mt-4">Grades submitted successfully!</p>}
            {submitError && <p className="text-center text-red-600 mt-4">{submitError}</p>}
        </div>
    );
}

export default EvaluationPage;
