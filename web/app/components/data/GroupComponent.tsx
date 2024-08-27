import React, {useState} from "react";
import {ProjectGroup} from "@/app/api/models/project-group";
import MinimalStudentComponent from "@/app/components/data/MinimalStudentComponent";
import {ProjectState} from "@/app/api/models/project-state";

async function updateGroupMark(group_id: string, mark: number) {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    await fetch(`${api_url}/groups/${group_id}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({ mark: mark }),
    });
}

const GroupComponent: React.FC<{
    info: ProjectGroup;
    withMarks: boolean;
    onDelete: (id: string) => void;
    projectState: ProjectState;
}> = ({ info, withMarks, onDelete, projectState }) => {
    const [groupMark, setGroupMark] = useState<number>(info.group.mark || 0);
    const [editMode, setEditMode] = useState<boolean>(false);
    const [markSubmitting, setMarkSubmitting] = useState<boolean>(false);
    const [markError, setMarkError] = useState<string | null>(null);

    const handleMarkSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setMarkSubmitting(true);
        setMarkError(null);

        try {
            await updateGroupMark(info.group.id, groupMark);
            setEditMode(false);
        } catch (error) {
            setMarkError("Failed to update the mark. Please try again.");
        } finally {
            setMarkSubmitting(false);
        }
    };

    return (
        <div className="relative p-6 border rounded-lg shadow-lg hover:shadow-xl transition-shadow bg-white flex flex-col space-y-4">
            <div className="flex justify-between items-center">
                <p className="text-xl font-semibold text-gray-800">{info.group.name.toUpperCase()}</p>
                {withMarks && !editMode && (
                    <p className="text-lg font-bold text-gray-700">{groupMark}</p>
                )}
            </div>
            <div className="space-y-2">
                {info.students.map((student) => (
                    <MinimalStudentComponent
                        key={student.student.id}
                        student_details={student}
                        withMark={withMarks}
                        group_id={info.group.id}
                        student_id={student.student.id}
                    />
                ))}
            </div>

            {projectState === ProjectState.Finished && (
                <div className="mt-4">
                    {editMode ? (
                        <form onSubmit={handleMarkSubmit} className="flex items-center space-x-4">
                            <input
                                type="number"
                                value={groupMark}
                                onChange={(e) => setGroupMark(Number(e.target.value))}
                                min="0"
                                max="20"
                                required
                                className="border rounded-lg px-3 py-2 w-24 text-gray-700 bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-400"
                            />
                            <button
                                type="submit"
                                className="px-4 py-2 bg-gray-700 text-white font-semibold rounded-lg shadow-md hover:bg-gray-800 transition"
                                disabled={markSubmitting}
                            >
                                {markSubmitting ? "Submitting..." : "Submit"}
                            </button>
                            <button
                                type="button"
                                className="px-4 py-2 bg-gray-400 text-white font-semibold rounded-lg shadow-md hover:bg-gray-500 transition"
                                onClick={() => setEditMode(false)}
                                disabled={markSubmitting}
                            >
                                Cancel
                            </button>
                        </form>
                    ) : (
                        <button
                            className="mt-2 px-4 py-2 bg-gray-700 text-white font-semibold rounded-lg shadow-md hover:bg-gray-800 transition"
                            onClick={() => setEditMode(true)}
                        >
                            Edit Mark
                        </button>
                    )}
                    {markError && <p className="text-red-500 mt-2">{markError}</p>}
                </div>
            )}
        </div>
    );
};

export default GroupComponent;
