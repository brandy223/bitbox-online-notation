'use client'

import React, {useEffect, useState} from "react";
import {hideModal} from "@/app/utils";
import {ProjectGroup} from "@/app/api/models/project-group";
import {Student} from "@/app/api/models/student";
import {FaSpinner} from "react-icons/fa";

interface NewGroupModalProps {
    groups: ProjectGroup[];
    setGroups: React.Dispatch<React.SetStateAction<ProjectGroup[]>>;
    project_id: string;
}

async function getStudentsWithoutGroups(project_id: string): Promise<Student[]> {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${api_url}/groups/project/${project_id}/students`, {
        headers: { "Content-Type": "application/json" },
        credentials: "include",
    });
    const data = await response.json();
    return data as Student[];
}

const fetchStudents = async (
    project_id: string,
    setStudents: React.Dispatch<React.SetStateAction<Student[]>>,
    setStudentsLoading: React.Dispatch<React.SetStateAction<boolean>>,
    setStudentsError: React.Dispatch<React.SetStateAction<string | null>>
) => {
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

const NewGroupModal: React.FC<NewGroupModalProps> = ({ groups, setGroups, project_id }) => {
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
        const { name, value } = e.target;
        setGroupFormData((prevData) => ({ ...prevData, [name]: value }));
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

            const group_response = await fetch(`${api_url}/groups/project/${project_id}`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
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
                headers: { "Content-Type": "application/json" },
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

            // Vérification avant mise à jour de l'état
            if (Array.isArray(groups)) {
                setGroups([...groups, newGroup]);
            } else {
                setError("An error occurred during the creation of the group. Please try again later.");
            }

            await fetchStudents(project_id, setStudents, setStudentsLoading, setStudentsError);

            hideModal("new_group_modal");
        } catch (err) {
            setError("An error occurred. Please try again later.");
        } finally {
            setLoading(false);
        }
    };


    return (
        <dialog id="new_group_modal" className="modal">
            <div className="modal-box relative p-6 bg-white rounded-lg shadow-lg">
                <button
                    type="button"
                    className="absolute top-3 right-3 text-gray-500 hover:text-gray-700"
                    onClick={() => hideModal("new_group_modal")}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none"
                         viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
                              d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
                <h3 className="text-xl font-semibold text-gray-800 mb-4">Créer un groupe</h3>
                <div className="divider"></div>
                {loading && (
                    <div className="flex items-center justify-center my-2">
                        <FaSpinner className="animate-spin text-blue-500 mr-2" />
                        <span className="text-gray-600">Création du groupe...</span>
                    </div>
                )}
                {error && (
                    <div className="my-2 p-2 bg-red-100 text-red-700 rounded">
                        {error}
                    </div>
                )}
                <form onSubmit={handleSubmit} className="flex flex-col space-y-4">
                    <div>
                        <label htmlFor="name" className="block text-gray-700 font-medium mb-1">
                            Nom du groupe
                        </label>
                        <input
                            type="text"
                            placeholder="Entrez le nom du groupe"
                            name="name"
                            id="name"
                            value={groupFormData.name}
                            required
                            maxLength={64}
                            onChange={handleInputChange}
                            className="w-full px-4 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />
                    </div>
                    <div>
                        <p className="text-gray-700 font-medium mb-2">Sélectionner les étudiants</p>
                        {studentsLoading ? (
                            <div className="flex items-center">
                                <FaSpinner className="animate-spin text-blue-500 mr-2" />
                                <span className="text-gray-600">Chargement des étudiants...</span>
                            </div>
                        ) : studentsError ? (
                            <div className="text-red-600">{studentsError}</div>
                        ) : students.length === 0 ? (
                            <div className="text-gray-600">Aucun étudiant disponible</div>
                        ) : (
                            <div className="max-h-60 overflow-y-auto p-2 border border-gray-200 rounded">
                                {students.map((student) => (
                                    <div key={student.id} className="flex items-center mb-2">
                                        <input
                                            type="checkbox"
                                            value={student.id}
                                            onChange={handleCheckboxChange}
                                            id={`student-${student.id}`}
                                            className="h-4 w-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                                        />
                                        <label htmlFor={`student-${student.id}`} className="ml-2 text-gray-700">
                                            {student.name}
                                        </label>
                                    </div>
                                ))}
                            </div>
                        )}
                    </div>
                    <div className="flex justify-end space-x-3">
                        <button
                            type="submit"
                            className={`px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 ${
                                loading ? 'opacity-50 cursor-not-allowed' : ''
                            }`}
                            disabled={loading}
                        >
                            {loading ? (
                                <>
                                    <FaSpinner className="animate-spin inline mr-2" />
                                    Submit...
                                </>
                            ) : (
                                "Soumettre"
                            )}
                        </button>
                        <button
                            type="button"
                            onClick={() => hideModal("new_group_modal")}
                            className="px-4 py-2 bg-gray-300 text-gray-700 rounded hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-500"
                        >
                            Cancel
                        </button>
                    </div>
                </form>
            </div>
            <form method="dialog" className="modal-backdrop" onClick={() => hideModal("new_group_modal")}>
                <button>close</button>
            </form>
        </dialog>
    )

}

export default NewGroupModal;
