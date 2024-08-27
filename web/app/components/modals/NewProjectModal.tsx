import React, {useState} from "react";
import {hideModal} from "@/app/utils";
import {Project} from "@/app/api/models/project";
import {ProjectState} from "@/app/api/models/project-state";

interface NewProjectModalProps {
    projects: Project[];
    setProjects: React.Dispatch<React.SetStateAction<Project[]>>;
    promotion_id: string;
}

const NewProjectModal: React.FC<NewProjectModalProps> = ({
                                                             projects,
                                                             setProjects,
                                                             promotion_id,
                                                         }) => {
    const [formData, setFormData] = useState({
        name: "",
        description: "",
        end_date: "",
        start_date: "",
        notation_period_duration: "",
    });

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setFormData((prevData) => ({ ...prevData, [name]: value }));
    };

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setError(null);
        setLoading(true);
        try {
            formData.start_date = formData.start_date + ":00";
            formData.end_date = formData.end_date + ":00";
            const notation_period_duration = parseInt(formData.notation_period_duration);

            const response = await fetch(
                `http://localhost:8080/api/projects/promotion/${promotion_id}`,
                {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({
                        name: formData.name,
                        description: formData.description,
                        end_date: formData.end_date,
                        start_date: formData.start_date,
                        notation_period_duration,
                    }),
                    credentials: "include",
                }
            );

            if (response.status === 201) {
                const id: string = await response.json();
                const newProject: Project = {
                    id,
                    name: formData.name,
                    description: formData.description,
                    end_date: formData.end_date,
                    start_date: formData.start_date,
                    notation_period_duration: formData.notation_period_duration as unknown as number,
                    promotion_id,
                    state: ProjectState.NotStarted,
                };
                setProjects([...projects, newProject]);
                hideModal("new_project_modal");
            } else {
                console.log(response);
                setError("An error occurred. Please try again later.");
            }
        } catch (err) {
            console.log(err);
            setError("An error occurred. Please try again later.");
        } finally {
            setLoading(false);
        }
    };

    return (
        <dialog id="new_project_modal" className="modal">
            <div className="modal-box max-w-md p-6 bg-gray-200 text-gray-600">
                <h3 className="font-bold text-xl mb-4">Create a Project</h3>
                <div className="divider mb-4"></div>
                {loading ? (
                    <div className="flex justify-center items-center mb-4">
                        <div className="loader"></div>
                        <span className="ml-2">Loading...</span>
                    </div>
                ) : error ? (
                    <p className="text-red-500 mb-4">{error}</p>
                ) : null}
                <form onSubmit={handleSubmit} className="flex flex-col space-y-4">
                    <div className="form-control">
                        <label htmlFor="name" className="label">
                            Project Name
                        </label>
                        <input
                            type="text"
                            placeholder="Project name"
                            name="name"
                            id="name"
                            value={formData.name}
                            required
                            maxLength={64}
                            onChange={handleInputChange}
                            className="input input-bordered w-full bg-gray-200"
                        />
                    </div>
                    <div className="form-control">
                        <label htmlFor="description" className="label">
                            Project Description
                        </label>
                        <input
                            type="text"
                            placeholder="Project description"
                            name="description"
                            id="description"
                            value={formData.description}
                            onChange={handleInputChange}
                            className="input input-bordered w-full bg-gray-200"
                        />
                    </div>
                    <div className="form-control">
                        <label htmlFor="start_date" className="label">
                            Start Date
                        </label>
                        <input
                            type="datetime-local"
                            placeholder="Start date"
                            name="start_date"
                            id="start_date"
                            value={formData.start_date}
                            required
                            onChange={handleInputChange}
                            className="input input-bordered w-full bg-gray-200"
                        />
                    </div>
                    <div className="form-control">
                        <label htmlFor="end_date" className="label">
                            End Date
                        </label>
                        <input
                            type="datetime-local"
                            placeholder="End date"
                            name="end_date"
                            id="end_date"
                            value={formData.end_date}
                            required
                            onChange={handleInputChange}
                            className="input input-bordered w-full bg-gray-200"
                        />
                    </div>
                    <div className="form-control">
                        <label htmlFor="notation_period_duration" className="label">
                            Notation Period Duration (in days)
                        </label>
                        <input
                            type="number"
                            placeholder="Notation period duration"
                            name="notation_period_duration"
                            id="notation_period_duration"
                            value={formData.notation_period_duration}
                            onChange={handleInputChange}
                            className="input input-bordered w-full bg-gray-200"
                        />
                    </div>
                    {/*<div className="flex justify-end mt-6">*/}
                    {/*    */}

                    {/*</div>*/}
                    <div className="flex justify-end space-x-4">
                        <button type="button" onClick={() => hideModal("new_project_modal")}
                                className="bg-gray-300 text-gray-700 rounded-lg px-4 py-2 hover:bg-gray-400 transition">
                            Cancel
                        </button>
                        <button type="submit"
                                className="bg-blue-500 text-white rounded-lg px-4 py-2 hover:bg-blue-600 transition"
                                disabled={loading}>
                            {loading ? "Submitting..." : "Submit"}
                        </button>
                    </div>
                </form>
            </div>
        </dialog>
    );
};

export default NewProjectModal;
