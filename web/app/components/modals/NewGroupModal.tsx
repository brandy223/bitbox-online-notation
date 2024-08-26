'use client'

import React, {useEffect, useState} from "react";
import {hideModal} from "@/app/utils";
import {ProjectGroup} from "@/app/api/models/project-group";
import {Student} from "@/app/api/models/student";

interface NewProjectModalProps {
    groups: ProjectGroup[];
    setGroups: React.Dispatch<React.SetStateAction<ProjectGroup[]>>;
    project_id: string;
}

async function getStudentsWithoutGroups(project_id: string): Promise<Student[]> {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${api_url}/groups/project/${project_id}/students`, {
        headers: {"Content-Type": "application/json"},
        credentials: "include",
    });
    const data = await response.json();
    return data as Student[];
}

const fetchStudents = async (project_id: string, setStudents: React.Dispatch<React.SetStateAction<Student[]>>, setStudentsLoading: React.Dispatch<React.SetStateAction<boolean>>, setStudentsError: React.Dispatch<React.SetStateAction<string | null>>) => {
    setStudentsLoading(true);
    setStudentsError(null);
    try {
        const students = await getStudentsWithoutGroups(project_id);
        setStudents(students);
    } catch (err) {
        setStudentsError("An error occurred. Please try again later.");
    } finally {
        setStudentsLoading(false);
    }
}

const NewGroupModal: React.FC<NewProjectModalProps> = ({groups, setGroups, project_id}) => {
    const [groupFormData, setGroupFormData] = useState({
        name: "",
    });
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const [studentsIds, setStudentsIds] = useState<string[]>([]);
    const [students, setStudents] = useState<Student[]>([]);

    const [studentsLoading, setStudentsLoading] = useState(false);
    const [studentsError, setStudentsError] = useState<string | null>(null);

    useEffect(() => {
        fetchStudents(project_id, setStudents, setStudentsLoading, setStudentsError);
    }, [project_id]);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const {name, value} = e.target;
        setGroupFormData((prevData) => ({...prevData, [name]: value}));
    };

    const handleCheckboxChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { value, checked } = e.target;
        setStudentsIds((prevIds) =>
            checked ? [...prevIds, value] : prevIds.filter((id) => id !== value)
        );
    };

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setError(null);
        setLoading(true);
        try {
            const api_url = process.env.NEXT_PUBLIC_API_URL;

            // Create group
            const group_response = await fetch(`${api_url}/groups/project/${project_id}`, {
                method: "POST",
                headers: {"Content-Type": "application/json"},
                body: JSON.stringify(groupFormData),
                credentials: "include",
            });

            if (group_response.status !== 201) {
                setError("An error occurred. Please try again later.");
                return;
            }

            const group_id: string = await group_response.json();

            const students_response = await fetch(`${api_url}/groups/${group_id}/students`, {
                method: "POST",
                headers: {"Content-Type": "application/json"},
                body: JSON.stringify(studentsIds),
                credentials: "include",
            });

            if (students_response.status !== 200) {
                setError("An error occurred. Please try again later.");
                return;
            }

            const newGroup: ProjectGroup = {
                group: {
                    id: group_id,
                    max_mark: 20,
                    name: groupFormData.name,
                    project_id: project_id,
                },
                students: students
                    .filter((student) => studentsIds.includes(student.id))
                    .map((student) => ({
                        mark: null,
                        student: student,
                    })),
            }

            // Check if groups is an array before setting state
            if (Array.isArray(groups)) {
                setGroups([...groups, newGroup]);
            } else {
                setError("An error occurred while updating the groups. Please try again.");
            }

            fetchStudents(project_id, setStudents, setStudentsLoading, setStudentsError);

            hideModal("new_group_modal");
        } catch (err) {
            setError("An error occurred. Please try again later.");
        } finally {
            setLoading(false);
        }
    };


    return (
        <dialog id="new_group_modal" className="modal">
            <div className="modal-box">
                <h3 className="font-bold text-lg">Create a group</h3>
                <div className="divider"></div>
                {loading ? (
                    <p>Loading...</p>
                ) : error ? (
                    <p style={{color: "red"}}>{error}</p>
                ): null}
                <form onSubmit={handleSubmit} className={"flex flex-col"}>
                    <input
                        type="text"
                        placeholder="Group name"
                        name="name"
                        id="name"
                        value={groupFormData.name}
                        required
                        maxLength={64}
                        onChange={handleInputChange}
                        className="input input-bordered w-full max-w-xs"
                    />

                    <div className="my-4">
                        {studentsLoading ? (
                            <p>Loading students...</p>
                        ) : studentsError ? (
                            <p style={{color: "red"}}>{studentsError}</p>
                        ) : ( students.length === 0 ? (
                            <p>No students available</p>
                        ) :
                            students.map((student) => (
                                <div key={student.id} className="form-control">
                                    <label className="cursor-pointer label">
                                        <span className="label-text">{student.name}</span>
                                        <input
                                            type="checkbox"
                                            value={student.id}
                                            onChange={handleCheckboxChange}
                                            className="checkbox checkbox-primary"
                                        />
                                    </label>
                                </div>
                            ))
                        )}
                    </div>

                    <div className="my-4 flex justify-end w-full">
                        <button type={"submit"}
                                className={"rounded-full hover:bg-base-200 px-4 py-2 font-bold w-32"}>
                            Submit
                        </button>
                        <button onClick={() => hideModal("new_group_modal")}
                                className={"rounded-full hover:bg-base-200 px-4 py-2 font-bold w-32"}>
                            Cancel
                        </button>
                    </div>
                </form>
            </div>
            <form method="dialog" className="modal-backdrop">
                <button>close</button>
            </form>
        </dialog>
    )
}

export default NewGroupModal;