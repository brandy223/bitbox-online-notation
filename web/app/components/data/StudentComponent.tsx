import React, {useState} from "react";
import {Student} from "@/app/api/models/student";
import {FaCheck, FaEdit, FaTimes} from "react-icons/fa";
import {capitalizeFirstLetter} from "@/app/utils";

const StudentComponent: React.FC<{
    student: Student,
    onDelete: (id: string) => void,
    onUpdate: (id: string, updatedStudent: Partial<Student>) => void
}> = ({ student, onDelete, onUpdate }) => {
    const [isEditing, setIsEditing] = useState(false);
    const [editedStudent, setEditedStudent] = useState(student);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setEditedStudent(prev => ({ ...prev, [name]: value }));
    };

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        onUpdate(student.id, editedStudent);
        setIsEditing(false);
    };

    return (
        <div className="relative px-4 py-2 border rounded-lg shadow-sm hover:shadow-md transition-shadow group">
            <button
                onClick={() => onDelete(student.id)}
                className="absolute top-2 left-2 text-red-500 hover:text-red-700 opacity-0 group-hover:opacity-100 transition-opacity"
                title="Delete student"
            >
                <FaTimes />
            </button>

            {isEditing ? (
                <form onSubmit={handleSubmit} className="mt-6">
                    <input
                        type="text"
                        name="name"
                        value={editedStudent.name}
                        maxLength={64}
                        onChange={handleInputChange}
                        className="block w-full mb-2 px-2 py-1 border rounded"
                    />
                    <input
                        type="text"
                        name="surname"
                        value={editedStudent.surname}
                        maxLength={64}
                        onChange={handleInputChange}
                        className="block w-full mb-2 px-2 py-1 border rounded"
                    />
                    <input
                        type="email"
                        name="email"
                        value={editedStudent.email}
                        maxLength={128}
                        onChange={handleInputChange}
                        className="block w-full mb-2 px-2 py-1 border rounded"
                    />
                    <button
                        type="submit"
                        className="bg-green-500 text-white px-2 py-1 rounded hover:bg-green-600"
                    >
                        <FaCheck /> Save
                    </button>
                </form>
            ) : (
                <>
                    <p className="font-bold">{student.name.toUpperCase()}</p>
                    <p>{capitalizeFirstLetter(student.surname)}</p>
                </>
            )}

            {!isEditing && (
                <button
                    onClick={() => setIsEditing(true)}
                    className="absolute bottom-2 left-2 text-blue-500 hover:text-blue-700 opacity-0 group-hover:opacity-100 transition-opacity"
                    title="Edit student"
                >
                    <FaEdit /> Edit
                </button>
            )}
        </div>
    );
};


export default StudentComponent;